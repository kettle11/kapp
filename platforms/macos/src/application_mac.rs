use super::apple::*;
use super::window_mac::*;
use crate::{Event, PlatformApplicationTrait, PlatformEventLoopTrait, WindowId, WindowParameters};
use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

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
    ns_application: *mut Object,
    pub modifier_flags: u64, // Key modifier flags
    pub produce_event_callback: Option<Box<dyn FnMut(Event)>>,
    pub events: Rc<RefCell<Vec<Event>>>,
    requested_redraw: Vec<WindowId>,
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

extern "C" fn control_flow_end_handler(
    _: CFRunLoopObserverRef,
    _: CFRunLoopActivity,
    _: *mut std::ffi::c_void,
) {
    // This is called after all events in the event loop are processed.
    let (mut callback, events) = APPLICATION_DATA.with(|d| {
        let mut application_data = d.borrow_mut();

        (
            application_data
                .as_mut()
                .unwrap()
                .produce_event_callback
                .take()
                .unwrap(),
            application_data.as_mut().unwrap().events.clone(),
        )
    });

    // Process all queued events.
    // The reason that events are queued is because the callback can call
    // application calls that produce new events which would cause the
    // callback to be called again within the same stack.
    // This loop is structured this way so that `events` is not borrowed while
    // the callback is called.
    // `events` may be appended to from code within the callback.
    let mut event = events.borrow_mut().pop();
    while let Some(e) = event {
        callback(e);
        event = events.borrow_mut().pop();
    }

    // Now process all redraw request events
    APPLICATION_DATA.with(|d| {
        let mut application_data = d.borrow_mut();

        for window_id in &application_data.as_mut().unwrap().requested_redraw {
            unsafe {
                let window_view: *mut Object =
                    msg_send![window_id.raw() as *mut Object, contentView];
                let () = msg_send![window_view, setNeedsDisplay: YES];
            }
        }

        application_data.as_mut().unwrap().produce_event_callback = Some(callback);
    });
}

pub struct PlatformEventLoop {
    ns_application: *mut Object,
}

impl PlatformEventLoopTrait for PlatformEventLoop {
    fn run(&mut self, callback: Box<dyn FnMut(crate::Event)>) {
        APPLICATION_DATA.with(|d| {
            let mut application_data = d.borrow_mut();
            application_data.as_mut().unwrap().produce_event_callback = Some(Box::new(callback));
        });

        unsafe {
            let () = msg_send![self.ns_application, run];
        }
    }
}

pub struct PlatformApplication {
    // application_data: Rc<RefCell<ApplicationData>>,
    window_class: *const objc::runtime::Class,
    view_class: *const objc::runtime::Class,
    ns_application: *mut Object,
    _run_loop_custom_event_source: CFRunLoopSourceRef,
}

impl PlatformApplicationTrait for PlatformApplication {
    type EventLoop = PlatformEventLoop;

    fn new() -> Self {
        unsafe {
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
                ns_application,
                modifier_flags: 0,
                produce_event_callback: None,
                requested_redraw: Vec::new(),
                events: Rc::new(RefCell::new(Vec::new())),
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

            Self {
                window_class: window_delegate_declaration(),
                view_class: view_delegate_declaration(),
                ns_application,
                _run_loop_custom_event_source: run_loop_custom_event_source,
            }
        }
    }

    fn event_loop(&mut self) -> Self::EventLoop {
        PlatformEventLoop {
            ns_application: self.ns_application,
        }
    }

    fn set_window_position(&mut self, window_id: &WindowId, x: u32, y: u32) {
        unsafe {
            let screen: *const Object = msg_send![window_id.raw() as *mut Object, screen];
            let screen_frame: CGRect = msg_send![screen, frame];

            let backing_scale: CGFloat =
                msg_send![window_id.raw() as *mut Object, backingScaleFactor];
            let () =
                msg_send![
                    window_id.raw() as *mut Object,
                    setFrameTopLeftPoint: NSPoint::new((x as f64) / backing_scale, screen_frame.size.height - (y as f64) / backing_scale)];
        }
    }
    fn set_window_dimensions(&mut self, window_id: &WindowId, width: u32, height: u32) {
        unsafe {
            let backing_scale: CGFloat =
                msg_send![window_id.raw() as *mut Object, backingScaleFactor];
            let () =
                msg_send![window_id.raw() as *mut Object, setContentSize: NSSize::new((width as f64) / backing_scale, (height as f64) / backing_scale)];
        }
    }
    fn set_window_title(&mut self, window_id: &WindowId, title: &str) {
        unsafe {
            let title = NSString::new(&title);
            let () = msg_send![window_id.raw() as *mut Object, setTitle: title.raw];
        }
    }
    fn minimize_window(&mut self, window_id: &WindowId) {
        unsafe {
            let () = msg_send![window_id.raw() as *mut Object, miniaturize: nil];
        }
    }
    fn maximize_window(&mut self, _window_id: &WindowId) {
        // Not implemented on Mac
        // There is no analogous behavior?
    }
    fn fullscreen_window(&mut self, window_id: &WindowId) {
        unsafe {
            let () = msg_send![window_id.raw() as *mut Object, toggleFullScreen: nil];
        }
    }
    fn restore_window(&mut self, _window_id: &WindowId) {
        unimplemented!()
    }
    fn close_window(&mut self, _window_id: &WindowId) {
        unimplemented!()
    }
    fn redraw_window(&mut self, window_id: &WindowId) {
        let in_live_resize: bool =
            unsafe { msg_send![window_id.raw() as *mut Object, inLiveResize] };

        // If resizing the window don't send a redraw request as it will get one
        // anyways
        if !in_live_resize {
            APPLICATION_DATA.with(|d| {
                let mut application_data = d.borrow_mut();
                application_data
                    .as_mut()
                    .unwrap()
                    .requested_redraw
                    .push(*window_id);
            });
        }
    }
    fn set_mouse_position(&mut self, _x: u32, _y: u32) {
        // Need to account for backing scale here!

        /*
        CGWarpMouseCursorPosition(CGPoint {
            x: x as f64,
            y: y as f64,
        });
        */

        unimplemented!()
    }
    fn new_window(&mut self, window_parameters: &WindowParameters) -> WindowId {
        let result =
            super::window_mac::build(window_parameters, self.window_class, self.view_class);
        result.unwrap()
    }
    fn quit(&mut self) {
        unsafe {
            let ns_application =
                APPLICATION_DATA.with(|d| d.borrow_mut().as_mut().unwrap().ns_application);
            let () = msg_send![ns_application, terminate: nil];
        }
    }
}
