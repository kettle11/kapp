use super::apple::*;
use super::window_mac::*;
use crate::application_message::{ApplicationMessage, ApplicationMessage::*};
use crate::Application;
use crate::Event;
use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;
use std::sync::mpsc;

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

// Global singleton data shared by all windows and the application struct.
pub struct ApplicationData {
    // Used to construct a new window
    pub window_class: *const objc::runtime::Class,
    pub view_class: *const objc::runtime::Class,

    run_loop_custom_event_source: CFRunLoopSourceRef,

    frame_requested: bool,
    ns_application: *mut Object,
    // pub program_callback: Option<Box<ProgramCallback>>,
    pub modifier_flags: u64, // Key modifier flags
    /// This is only used if an event is produced within the program_callback.
    /// For example if a window is minimized it produces a minimized event in the same
    /// call tree.
    pub event_queue: Vec<Event>,
    program_to_application_receive: Option<mpsc::Receiver<ApplicationMessage>>,
    pub callback_event_channel: Option<mpsc::Sender<Event>>,
    pub windows: Vec<Box<InnerWindowData>>,
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

fn initialize_application(program_to_application_receive: mpsc::Receiver<ApplicationMessage>) {}

pub fn process_events(application: &Rc<RefCell<ApplicationData>>) {
    unsafe {
        let events = application
            .borrow_mut()
            .program_to_application_receive
            .take()
            .unwrap();
        for event in events.try_recv() {
            match event {
                MinimizeWindow { window } => {
                    let () = msg_send![window.inner_window(), miniaturize: nil];
                }
                SetWindowPosition { window, x, y } => {
                    let backing_scale: CGFloat =
                        msg_send![window.inner_window(), backingScaleFactor];
                    let () =
                        msg_send![window.inner_window(), setFrameOrigin: NSPoint::new((x as f64) / backing_scale, (y as f64) / backing_scale)];
                }
                SetWindowSize {
                    window,
                    width,
                    height,
                } => {
                    let backing_scale: CGFloat =
                        msg_send![window.inner_window(), backingScaleFactor];
                    let () =
                        msg_send![window.inner_window(), setContentSize: NSSize::new((width as f64) / backing_scale, (height as f64) / backing_scale)];
                }
                MaximizeWindow { .. } => {}
                FullscreenWindow { window } => {
                    let () = msg_send![window.inner_window(), toggleFullScreen: nil];
                }
                RestoreWindow { .. } => unimplemented!(),
                DropWindow { .. } => unimplemented!(),
                RequestFrame { .. } => application.borrow_mut().frame_requested = true,
                SetMousePosition { x, y } => {
                    CGWarpMouseCursorPosition(CGPoint {
                        x: x as f64,
                        y: y as f64,
                    });
                }
                Quit => {
                    let ns_application = application.borrow().ns_application;
                    let () = msg_send![ns_application, terminate: nil];
                }
                NewWindow {
                    window_parameters,
                    response_channel,
                } => {
                    // This won't work because the application is already borrowed as mutable.
                    let result = super::window_mac::build(
                        window_parameters,
                        &mut application.borrow_mut(),
                        application,
                    );
                    response_channel.send(result).unwrap();
                }
            }
        }

        application.borrow_mut().program_to_application_receive = Some(events);
    }
}

// At the end of a frame produce a draw event.
extern "C" fn control_flow_end_handler(
    _: CFRunLoopObserverRef,
    _: CFRunLoopActivity,
    observer_context_info: *mut std::ffi::c_void,
) {
    // println!("End handler");
    unsafe {
        let application = &*(observer_context_info as *mut Rc<RefCell<ApplicationData>>);

        // Check for events
        process_events(&application);
        let mut application_data = application.borrow_mut();

        if application_data.frame_requested {
            if let Some(channel) = application_data.callback_event_channel.as_mut() {
                channel.send(Event::Draw).unwrap();
            }
            application_data.frame_requested = false;
        }
    }
}

pub struct PlatformApplication {
    application_data: Rc<RefCell<ApplicationData>>,
    ns_application: *mut Object,
}

impl PlatformApplication {
    pub fn new(
        program_to_application_receive: mpsc::Receiver<
            crate::application_message::ApplicationMessage,
        >,
    ) -> Self {
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

            let run_loop_custom_event_source = self::create_run_loop_source();

            let application_data = ApplicationData {
                window_class: window_delegate_declaration(),
                view_class: view_delegate_declaration(),
                frame_requested: true, // Always request an initial frame
                ns_application,
                modifier_flags: 0,
                event_queue: Vec::new(),
                callback_event_channel: None,
                windows: Vec::new(),
                run_loop_custom_event_source,
                program_to_application_receive: Some(program_to_application_receive),
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

            Self {
                ns_application,
                application_data,
            }
        }
    }

    pub fn flush_events(&mut self) {
        process_events(&self.application_data);
    }

    pub fn run<T>(self, application: &crate::Application, mut callback: T)
    where
        T: 'static + FnMut(&mut Application, crate::Event) + Send,
    {
        let (send_event, receiver_channel) = mpsc::channel();

        let (program_to_application_send, main_thread_id) = application.to_parts();
        // The PlatformApplication holds the data required to respond to events until
        // this function is called at which point it passes the callback and receive channel
        // to another thread.
        std::thread::spawn(move || {
            let mut application =
                Application::from_parts(program_to_application_send, main_thread_id);
            while let Ok(event) = receiver_channel.recv() {
                (callback)(&mut application, event)
            }
        });

        self.application_data.borrow_mut().callback_event_channel = Some(send_event);
        unsafe {
            let () = msg_send![self.ns_application, run];
        }

        println!("HERE");
    }
}
