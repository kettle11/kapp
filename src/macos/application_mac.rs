use super::apple::*;
use crate::Event;
use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

pub type ProgramCallback = dyn 'static + FnMut(Event);

static INSTANCE_DATA_IVAR_ID: &str = "instance_data";
static WINDOW_CLASS_NAME: &str = "KettlewinWindowClass";
static VIEW_CLASS_NAME: &str = "KettlewinViewClass";

pub struct Window {
    pub ns_view: *mut Object,
}

#[derive(Clone)]
pub struct Application {
    window_class: *const objc::runtime::Class,
    view_class: *const objc::runtime::Class,
    run_loop_custom_event_source: CFRunLoopSourceRef,
    application_data: Rc<RefCell<ApplicationData>>,
}

// Global singleton data shared by all windows and the application struct.
pub struct ApplicationData {
    frame_requested: bool,
    ns_application: *mut Object,
    pub program_callback: Option<Box<ProgramCallback>>,
    pub modifier_flags: u64, // Key modifier flags
}

// Information about a window delegate instance. Attached with an iVar.
pub struct WindowInstanceData {
    pub application_data: Rc<RefCell<ApplicationData>>,
    pub ns_window: *mut Object,
}

// Information about a view instance. Attached with an iVar.
// Perhaps this a bit redundant, it'd be better if there was a way to access the window's
// instance data.
pub struct ViewInstanceData {
    pub application_data: Rc<RefCell<ApplicationData>>,
}

pub fn get_window_instance_data(this: &Object) -> *mut WindowInstanceData {
    unsafe {
        let data: *mut c_void = *this.get_ivar(INSTANCE_DATA_IVAR_ID);
        data as *mut WindowInstanceData
    }
}

pub fn get_view_instance_data(this: &Object) -> *mut ViewInstanceData {
    unsafe {
        let data: *mut c_void = *this.get_ivar(INSTANCE_DATA_IVAR_ID);
        data as *mut ViewInstanceData
    }
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
                    application_data.frame_requested = frame_requested;
                    application_data.program_callback = program_callback;
                }
            }
            let application_data = ApplicationData {
                frame_requested: true, // Always request an initial frame
                ns_application,
                program_callback: None,
                modifier_flags: 0,
            };

            let application_data = Rc::new(RefCell::new(application_data));

            // This box that is leaked needs a way to be deallocated later.
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

pub struct WindowBuilder<'a> {
    application: &'a Application,
    pub position: Option<(u32, u32)>,
    pub dimensions: Option<(u32, u32)>,
    pub resizable: bool,
    pub title: Option<&'a str>,
}

impl<'a> WindowBuilder<'a> {
    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn resizable(&mut self, resizable: bool) -> &mut Self {
        self.resizable = resizable;
        self
    }

    /// Places the lower left corner of the window.
    pub fn position(&mut self, x: u32, y: u32) -> &mut Self {
        self.position = Some((x, y));
        self
    }
    
    pub fn dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.dimensions = Some((width, height));
        self
    }

    pub fn build(&self) -> Result<Window, ()> {
        unsafe {
            let (width, height) = self.dimensions.map_or((600., 600.), |(width, height)| {
                (width as f64, height as f64)
            });
            let rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(width, height));

            let mut style = NSWindowStyleMaskTitled
                | NSWindowStyleMaskClosable
                | NSWindowStyleMaskMiniaturizable;
            if self.resizable {
                style |= NSWindowStyleMaskResizable;
            }

            // Everything alloc-ed needs to be released somehow.
            let ns_window: *mut Object = msg_send![class!(NSWindow), alloc];
            let () = msg_send![
                ns_window,
                initWithContentRect:rect
                styleMask:style
                backing:NSBackingStoreBuffered
                defer:NO
            ];

            if let Some(position) = self.position {
                let () = msg_send![ns_window, cascadeTopLeftFromPoint:NSPoint::new(position.0 as f64, position.1 as f64)];
            } else {
                // Center the window if no position is specified.
                let () = msg_send![ns_window, center];
            }

            let title = self.title.unwrap_or("Untitled");
            let title = NSString::new(title);
            let () = msg_send![ns_window, setTitle: title.raw];
            let () = msg_send![ns_window, makeKeyAndOrderFront: nil];

            // setup view
            let ns_view: *mut Object = msg_send![self.application.view_class, alloc];

            // Heap allocate a data structure for the view.
            // Because this data is leaked it must be cleaned up manually later.
            let view_instance_data = Box::leak(Box::new(ViewInstanceData {
                application_data: Rc::clone(&self.application.application_data),
            })) as *mut ViewInstanceData as *mut c_void;
            (*ns_view).set_ivar(INSTANCE_DATA_IVAR_ID, view_instance_data);

            // Apparently this defaults to YES even without this call
            let () = msg_send![ns_view, setWantsBestResolutionOpenGLSurface: YES];

            // Setup a tracking area to receive mouse events within
            let tracking_area: *mut Object = msg_send![class!(NSTrackingArea), alloc];
            let () = msg_send![
                tracking_area,
                initWithRect: rect
                options: NSTrackingMouseEnteredAndExited | NSTrackingMouseMoved | NSTrackingActiveInKeyWindow
                owner: ns_view
                userInfo:nil];
            let () = msg_send![ns_view, addTrackingArea: tracking_area];

            // Setup window delegate that receives events.
            let ns_window_delegate: *mut Object = msg_send![self.application.window_class, new];

            // Heap allocate a data structure for the window.
            // Because this data is leaked it must be cleaned up manually later.
            let window_instance_data = Box::leak(Box::new(WindowInstanceData {
                application_data: Rc::clone(&self.application.application_data),
                ns_window,
            })) as *mut WindowInstanceData as *mut c_void;

            (*ns_window_delegate).set_ivar(INSTANCE_DATA_IVAR_ID, window_instance_data);
            let () = msg_send![ns_window, setDelegate: ns_window_delegate];
            let () = msg_send![ns_window, setContentView: ns_view];
            let () = msg_send![ns_window, makeFirstResponder: ns_view];

            let window = Window { ns_view };
            Ok(window)
        }
    }
}

impl Application {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder {}
    }

    pub fn new_window<'a>(&'a mut self) -> WindowBuilder<'a> {
        WindowBuilder {
            application: self,
            position: None,
            dimensions: None,
            resizable: true,
            title: None,
        }
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

    // Wakes up the run loop giving it a chance to send a draw event at the end of the frame.
    fn wake_run_loop(&self) {
        unsafe {
            CFRunLoopSourceSignal(self.run_loop_custom_event_source); // This line may not even be necessary.
            let run_loop = CFRunLoopGetMain();
            CFRunLoopWakeUp(run_loop);
        }
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
