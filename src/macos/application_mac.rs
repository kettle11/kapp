use super::apple::*;
use super::window_mac::*;
use crate::Event;
use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

pub type ProgramCallback = dyn 'static + FnMut(Event);

pub static INSTANCE_DATA_IVAR_ID: &str = "instance_data";
static WINDOW_CLASS_NAME: &str = "KettlewinWindowClass";
static VIEW_CLASS_NAME: &str = "KettlewinViewClass";
static APPLICATION_CLASS_NAME: &str = "KettlewinApplicationClass";

pub fn get_window_data(this: &Object) -> &mut InnerWindowData {
    unsafe {
        let data: *mut std::ffi::c_void = *this.get_ivar(INSTANCE_DATA_IVAR_ID);
        &mut *(data as *mut InnerWindowData)
    }
}

#[derive(Clone)]
pub struct Application {
    window_class: *const objc::runtime::Class,
    view_class: *const objc::runtime::Class,
    run_loop_custom_event_source: CFRunLoopSourceRef,
    application_data: Rc<RefCell<ApplicationData>>,
}

impl Application {
    pub fn get_window_class(&self) -> *const objc::runtime::Class {
        self.window_class
    }

    pub fn get_view_class(&self) -> *const objc::runtime::Class {
        self.view_class
    }

    pub fn get_application_data(&self) -> &Rc<RefCell<ApplicationData>> {
        &self.application_data
    }
}

// Global singleton data shared by all windows and the application struct.
pub struct ApplicationData {
    frame_requested: bool,
    ns_application: *mut Object,
    pub program_callback: Option<Box<ProgramCallback>>,
    pub modifier_flags: u64, // Key modifier flags
    /// This is only used if an event is produced within the program_callback.
    /// For example if a window is minimized it produces a minimized event in the same
    /// call tree.
    pub event_queue: Vec<Event>,
}
pub struct ApplicationInstanceData {
    pub application_data: Rc<RefCell<ApplicationData>>,
}

fn window_delegate_declaration() -> *const objc::runtime::Class {
    let superclass = class!(NSResponder);
    let mut decl = ClassDecl::new(WINDOW_CLASS_NAME, superclass).unwrap();
    super::events_mac::add_window_events_to_decl(&mut decl);
    decl.add_ivar::<*mut c_void>(INSTANCE_DATA_IVAR_ID);
    decl.register()
}

fn view_delegate_declaration() -> *const objc::runtime::Class {
    let superclass = class!(NSView);
    let mut decl = ClassDecl::new(VIEW_CLASS_NAME, superclass).unwrap();
    super::events_mac::add_view_events_to_decl(&mut decl);
    decl.add_ivar::<*mut c_void>(INSTANCE_DATA_IVAR_ID);
    decl.register()
}

fn application_delegate_declaration() -> *const objc::runtime::Class {
    let superclass = class!(NSResponder);
    let mut decl = ClassDecl::new(APPLICATION_CLASS_NAME, superclass).unwrap();
    super::events_mac::add_application_events_to_decl(&mut decl);
    decl.add_ivar::<*mut c_void>(INSTANCE_DATA_IVAR_ID);
    decl.register()
}

pub struct ApplicationBuilder {}
impl ApplicationBuilder {
    pub fn build(&self) -> Result<Application, ()> {
        unsafe {
            // let pool: *mut Object = unsafe { msg_send![class!(NSAutoreleasePool), new] };

            let ns_application: *mut Object = msg_send![class!(NSApplication), sharedApplication];

            let () = msg_send![
                ns_application,
                setActivationPolicy:
                    NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular
            ];

            // Setup the application delegate to handle application events.
            let ns_application_delegate_class = application_delegate_declaration();
            let ns_application_delegate: *mut Object =
                msg_send![ns_application_delegate_class, new];

            let () = msg_send![ns_application, setDelegate: ns_application_delegate];

            // At the end of a frame produce a draw event.
            extern "C" fn control_flow_end_handler(
                _: CFRunLoopObserverRef,
                _: CFRunLoopActivity,
                observer_context_info: *mut std::ffi::c_void,
            ) {
                unsafe {
                    // This is a little awkward, but the application_data cannot be borrowed
                    // while the program_callback is called as it may call functions that borrow application_data
                    let (frame_requested, mut program_callback) = {
                        let mut application_data = (*(observer_context_info
                            as *mut Rc<RefCell<ApplicationData>>))
                            .borrow_mut();
                        let frame_requested = application_data.frame_requested;
                        application_data.frame_requested = false;
                        (frame_requested, application_data.program_callback.take())
                    };

                    if frame_requested {
                        if let Some(program_callback) = program_callback.as_mut() {
                            program_callback(Event::Draw);
                        }
                    }

                    let mut application_data = (*(observer_context_info
                        as *mut Rc<RefCell<ApplicationData>>))
                        .borrow_mut();
                    application_data.program_callback = program_callback;
                }
            }

            let application_data = ApplicationData {
                frame_requested: true, // Always request an initial frame
                ns_application,
                program_callback: None,
                modifier_flags: 0,
                event_queue: Vec::new(),
            };

            let application_data = Rc::new(RefCell::new(application_data));

            // This allocation will persist until the program is quit.
            let application_instance_data = Box::leak(Box::new(Rc::clone(&application_data)))
                as *mut Rc<RefCell<ApplicationData>>
                as *mut c_void;
            (*ns_application_delegate).set_ivar(INSTANCE_DATA_IVAR_ID, application_instance_data);

            // This allocation will persist until the program is quit.
            let observer_context_info = Box::leak(Box::new(Rc::clone(&application_data)))
                as *mut Rc<RefCell<ApplicationData>>
                as *mut c_void;

            // We only used this context to pass application_data to the observer
            // The values in this data structure will be copied out.
            let observer_context = CFRunLoopObserverContext {
                copyDescription: std::ptr::null(),
                info: observer_context_info,
                release: std::ptr::null(),
                version: 0,
                retain: std::ptr::null(),
            };

            let observer = CFRunLoopObserverCreate(
                std::ptr::null_mut(),
                kCFRunLoopBeforeWaiting,
                YES,                  // Indicates we want this to run repeatedly
                CFIndex::min_value(), // The lower the value, the sooner this will run
                control_flow_end_handler,
                &observer_context as *const CFRunLoopObserverContext,
            );
            CFRunLoopAddObserver(CFRunLoopGetMain(), observer, kCFRunLoopCommonModes);

            let run_loop_custom_event_source = self::create_run_loop_source();

            let app = Application {
                window_class: window_delegate_declaration(),
                view_class: view_delegate_declaration(),
                run_loop_custom_event_source,
                application_data,
            };

            Ok(app)
        }
    }
}

fn create_run_loop_source() -> CFRunLoopSourceRef {
    extern "C" fn event_loop_proxy_handler(_: *mut std::ffi::c_void) {}

    unsafe {
        let rl = CFRunLoopGetMain();
        let mut context: CFRunLoopSourceContext = std::mem::zeroed();
        context.perform = Some(event_loop_proxy_handler);
        let source =
            CFRunLoopSourceCreate(std::ptr::null_mut(), CFIndex::max_value() - 1, &mut context);
        CFRunLoopAddSource(rl, source, kCFRunLoopCommonModes);
        CFRunLoopWakeUp(rl);
        source
    }
}

impl Application {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder {}
    }

    pub fn new_window<'a>(&'a mut self) -> WindowBuilder<'a> {
        WindowBuilder::new(self)
    }

    pub fn event_loop(&mut self) -> EventLoop {
        EventLoop {
            application_data: Rc::clone(&self.application_data),
        }
    }

    pub fn request_frame(&mut self) {
        let mut application_data = self.application_data.borrow_mut();
        application_data.frame_requested = true;

        self.wake_run_loop();
    }

    pub fn quit(&self) {
        let ns_application = self.application_data.borrow().ns_application;
        unsafe {
            let () = msg_send![ns_application, terminate: nil];
        }
    }

    pub fn set_mouse_position(&self, x: u32, y: u32) {
        unsafe {
            CGWarpMouseCursorPosition(CGPoint {
                x: x as f64,
                y: y as f64,
            });
        }
    }

    // Wakes up the run loop giving it a chance to send a draw event at the end of the frame.
    fn wake_run_loop(&self) {
        unsafe {
            CFRunLoopSourceSignal(self.run_loop_custom_event_source); // This line may not even be necessary.
            let run_loop = CFRunLoopGetMain();
            CFRunLoopWakeUp(run_loop);
        }
    }
}

// When the application is dropped, quit the program.
impl Drop for Application {
    fn drop(&mut self) {
        self.quit();
    }
}

pub struct EventLoop {
    application_data: Rc<RefCell<ApplicationData>>,
}

impl EventLoop {
    pub fn run<T>(&self, callback: T)
    where
        T: 'static + FnMut(crate::Event),
    {
        // The mutable borrow to application_data is dropped here before
        // the rest of the program runs.
        let ns_application = {
            let mut application_data = self.application_data.borrow_mut();
            application_data.program_callback = Some(Box::new(callback));
            application_data.ns_application
        };

        unsafe {
            let () = msg_send![ns_application, run];
        }
    }
}
