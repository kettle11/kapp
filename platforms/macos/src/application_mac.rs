use super::apple::*;
use super::window_mac::*;
use crate::{
    ApplicationMessage, Event, PlatformApplicationTrait, PlatformChannelTrait, PlatformWakerTrait,
};
use std::cell::RefCell;
use std::ffi::c_void;
use std::sync::mpsc;
use std::sync::mpsc::*;

pub static INSTANCE_DATA_IVAR_ID: &str = "instance_data";
static WINDOW_CLASS_NAME: &str = "KettlewinWindowClass";
static VIEW_CLASS_NAME: &str = "KettlewinViewClass";
static APPLICATION_CLASS_NAME: &str = "KettlewinApplicationClass";

thread_local!(pub static APPLICATION_DATA: RefCell<Option<Box<ApplicationData>>> = RefCell::new(None));

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

    frame_requested: bool,
    ns_application: *mut Object,
    // pub program_callback: Option<Box<ProgramCallback>>,
    pub modifier_flags: u64, // Key modifier flags
    program_to_application_receive: Option<mpsc::Receiver<ApplicationMessage>>,
    pub produce_event_callback: Option<Box<dyn FnMut(Event)>>,
    pub windows: Vec<Box<InnerWindowData>>,
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

pub fn process_events() {
    unsafe {
        let events = APPLICATION_DATA.with(|d| {
            d.borrow_mut()
                .as_mut()
                .unwrap()
                .program_to_application_receive
                .take()
                .unwrap()
        });

        while let Ok(event) = events.try_recv() {
            match event {
                ApplicationMessage::MinimizeWindow { window } => {
                    let () = msg_send![window.raw() as *mut Object, miniaturize: nil];
                }
                ApplicationMessage::SetWindowPosition { window, x, y } => {
                    let backing_scale: CGFloat =
                        msg_send![window.raw() as *mut Object, backingScaleFactor];
                    let () =
                        msg_send![window.raw() as *mut Object, setFrameOrigin: NSPoint::new((x as f64) / backing_scale, (y as f64) / backing_scale)];
                }
                ApplicationMessage::SetWindowSize {
                    window,
                    width,
                    height,
                } => {
                    let backing_scale: CGFloat =
                        msg_send![window.raw() as *mut Object, backingScaleFactor];
                    let () =
                        msg_send![window.raw() as *mut Object, setContentSize: NSSize::new((width as f64) / backing_scale, (height as f64) / backing_scale)];
                }
                ApplicationMessage::SetWindowTitle { window, title } => {
                    let title = NSString::new(&title);
                    let () = msg_send![window.raw() as *mut Object, setTitle: title.raw];
                }
                ApplicationMessage::MaximizeWindow { .. } => {}
                ApplicationMessage::FullscreenWindow { window } => {
                    let () = msg_send![window.raw() as *mut Object, toggleFullScreen: nil];
                }
                ApplicationMessage::RestoreWindow { .. } => unimplemented!(),
                ApplicationMessage::DropWindow { .. } => unimplemented!(),
                ApplicationMessage::RequestFrame { .. } => {
                    APPLICATION_DATA.with(|d| {
                        d.borrow_mut().as_mut().unwrap().frame_requested = true;
                    });
                }
                ApplicationMessage::SetMousePosition { x, y } => {
                    CGWarpMouseCursorPosition(CGPoint {
                        x: x as f64,
                        y: y as f64,
                    });
                }
                ApplicationMessage::Quit => {
                    let ns_application =
                        APPLICATION_DATA.with(|d| d.borrow_mut().as_mut().unwrap().ns_application);
                    let () = msg_send![ns_application, terminate: nil];
                }
                ApplicationMessage::NewWindow {
                    window_parameters,
                    response_channel,
                } => {
                    APPLICATION_DATA.with(|d| {
                        let mut application_data = d.borrow_mut();
                        let mut application_data = application_data.as_mut().unwrap();
                        let result =
                            super::window_mac::build(window_parameters, &mut application_data);
                        response_channel.send(result).unwrap();
                    });
                }
            }
        }

        APPLICATION_DATA.with(|d| {
            let mut application_data = d.borrow_mut();
            let application_data = application_data.as_mut().unwrap();

            application_data.program_to_application_receive = Some(events);
        });
    }
}

// At the end of a frame produce a draw event.
extern "C" fn control_flow_end_handler(
    _: CFRunLoopObserverRef,
    _: CFRunLoopActivity,
    _: *mut std::ffi::c_void,
) {
    // Check for events
    process_events();
    APPLICATION_DATA.with(|d| {
        let mut application_data = d.borrow_mut();
        let mut application_data = application_data.as_mut().unwrap();

        if application_data.frame_requested {
            if let Some(callback) = application_data.produce_event_callback.as_mut() {
                callback(Event::Draw);
            }
            application_data.frame_requested = false;
        }
    });
}

pub struct PlatformApplication {
    // application_data: Rc<RefCell<ApplicationData>>,
    ns_application: *mut Object,
    run_loop_custom_event_source: CFRunLoopSourceRef,
}

impl PlatformApplicationTrait for PlatformApplication {
    type Waker = PlatformWaker;
    type Channel = PlatformChannel;

    fn new() -> (Self::Channel, Self) {
        unsafe {
            let ns_application: *mut Object = msg_send![class!(NSApplication), sharedApplication];

            let () = msg_send![
                ns_application,
                setActivationPolicy:
                    NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular
            ];

            let (sender, receiver) = mpsc::channel();
            let platform_channel = Self::Channel { sender };

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
                windows: Vec::new(),
                program_to_application_receive: Some(receiver),
                produce_event_callback: None,
            };

            APPLICATION_DATA.with(|d| {
                *d.borrow_mut() = Some(Box::new(application_data));
            });

            // We only used this context to pass application_data to the observer
            // The values in this data structure will be copied out.
            let observer_context = CFRunLoopObserverContext {
                copyDescription: std::ptr::null(),
                info: std::ptr::null(),
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

            (
                platform_channel,
                Self {
                    ns_application,
                    // application_data,
                    run_loop_custom_event_source,
                },
            )
        }
    }

    fn flush_events(&mut self) {
        process_events();
    }

    fn run(&mut self, mut callback: Box<dyn FnMut(crate::Event) + Send>) {
        // User code is run on another thread because resize and certain other events block the main
        // thread on MacOS.
        // This method ensures a smooth experience.
        // 'run_raw' can be used if another method is required.
        let (send, receive) = std::sync::mpsc::channel();

        // When events are produced by the application send them to a channel
        let callback_wrapper = move |event| {
            send.send(event).unwrap();
        };

        // Receive the events from the channel and send them to the user code callback.
        std::thread::spawn(move || {
            while let Ok(event) = receive.recv() {
                callback(event);
            }
        });

        APPLICATION_DATA.with(|d| {
            let mut application_data = d.borrow_mut();
            application_data.as_mut().unwrap().produce_event_callback =
                Some(Box::new(callback_wrapper));
        });

        unsafe {
            let () = msg_send![self.ns_application, run];
        }
    }

    fn run_raw(&mut self, callback: Box<dyn FnMut(crate::Event) + Send>) {
        APPLICATION_DATA.with(|d| {
            let mut application_data = d.borrow_mut();
            application_data.as_mut().unwrap().produce_event_callback = Some(Box::new(callback));
        });

        unsafe {
            let () = msg_send![self.ns_application, run];
        }
    }

    fn get_waker(&self) -> Self::Waker {
        Self::Waker {
            run_loop_custom_event_source: self.run_loop_custom_event_source,
            main_thread_id: std::thread::current().id(),
        }
    }
}

#[derive(Clone)]
pub struct PlatformWaker {
    run_loop_custom_event_source: CFRunLoopSourceRef,
    main_thread_id: std::thread::ThreadId,
}

unsafe impl Send for PlatformWaker {}

impl PlatformWakerTrait for PlatformWaker {
    fn wake(&self) {
        unsafe {
            //CFRunLoopSourceSignal(self.run_loop_custom_event_source); // This line may not even be necessary.
            let run_loop = CFRunLoopGetMain();
            CFRunLoopWakeUp(run_loop);
        }
    }

    fn flush(&self) {
        if std::thread::current().id() == self.main_thread_id {
            process_events();
        } else {
            self.wake();
        }
    }
}

#[derive(Clone)]
pub struct PlatformChannel {
    sender: Sender<ApplicationMessage>,
}

impl PlatformChannelTrait for PlatformChannel {
    fn send(&mut self, message: ApplicationMessage) {
        self.sender.send(message).unwrap();
    }
}
