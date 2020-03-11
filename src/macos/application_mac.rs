use super::apple::*;
use crate::Event;

type Callback = dyn 'static + FnMut(Event);
pub static mut PROGRAM_CALLBACK: Option<Box<Callback>> = None;
pub static mut APP: Option<Box<Application>> = None;

pub struct Window {
    pub ns_view: *mut Object,
}

#[derive(Clone)]
pub struct Application {
    pub app: *mut Object,
    window_delegate_class: *const objc::runtime::Class,
    view_delegate_class: *const objc::runtime::Class,
    frame_requested: bool,
    run_loop_custom_event_source: CFRunLoopSourceRef,
}

/*
fn get_window_data(this: &Object) -> *mut Object {
    unsafe {
        let data = *this.get_ivar("window_data");
        data
    }
}
*/

fn window_delegate_declaration() -> *const objc::runtime::Class {
    let superclass = class!(NSResponder);
    let mut decl = ClassDecl::new("KettlewinWindowClass", superclass).unwrap();
    super::events_mac::add_window_events_to_decl(&mut decl);

    decl.add_ivar::<*mut Object>("window_data");
    decl.register()
}

fn view_delegate_declaration() -> *const objc::runtime::Class {
    let superclass = class!(NSView);
    let mut decl = ClassDecl::new("KettlewinViewClass", superclass).unwrap();
    super::events_mac::add_view_events_to_decl(&mut decl);
    decl.register()
}

pub struct ApplicationBuilder {}
impl ApplicationBuilder {
    pub fn build(&self) -> Result<Application, ()> {
        unsafe {
            // let pool: *mut Object = unsafe { msg_send![class!(NSAutoreleasePool), new] };

            let app: *mut Object = msg_send![class!(NSApplication), sharedApplication];
            let () = msg_send![
                app,
                setActivationPolicy:
                    NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular
            ];

            // At the end of a frame produce a draw event.
            extern "C" fn control_flow_end_handler(
                _: CFRunLoopObserverRef,
                _: CFRunLoopActivity,
                _: *mut std::ffi::c_void,
            ) {
                unsafe {
                    if let Some(app) = APP.as_mut() {
                        if app.frame_requested {
                            app.frame_requested = false;
                            super::events_mac::produce_event(Event::Draw);
                        }
                    }
                }
            }

            // Setup a runloop observer (Idea borrowed from Winit)
            let observer = CFRunLoopObserverCreate(
                std::ptr::null_mut(),
                kCFRunLoopBeforeWaiting,
                YES,                  // Indicates we want this to run repeatedly
                CFIndex::min_value(), // The lower the value, the sooner this will run
                control_flow_end_handler,
                std::ptr::null_mut(),
            );
            CFRunLoopAddObserver(CFRunLoopGetMain(), observer, kCFRunLoopCommonModes);

            let run_loop_custom_event_source = self::create_run_loop_source();

            let app = Application {
                app,
                window_delegate_class: window_delegate_declaration(),
                view_delegate_class: view_delegate_declaration(),
                frame_requested: true,
                run_loop_custom_event_source,
            };
            APP = Some(Box::new(app.clone()));
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

            // It appears these flags are deprecated, but the Rust wrapper does not expose the nondepcrated version?
            let mut style = NSWindowStyleMaskTitled
                | NSWindowStyleMaskClosable
                | NSWindowStyleMaskMiniaturizable;
            if self.resizable {
                style |= NSWindowStyleMaskResizable;
            }

            // Needs to be released somehow

            let window: *mut Object = msg_send![class!(NSWindow), alloc];
            let () = msg_send![
                window,
                initWithContentRect:rect
                styleMask:style
                backing:NSBackingStoreBuffered
                defer:NO
            ];

            if let Some(position) = self.position {
                let () = msg_send![window, cascadeTopLeftFromPoint:NSPoint::new(position.0 as f64, position.1 as f64)];
            } else {
                // Center the window
                let () = msg_send![window, center];
            }

            let title = self.title.unwrap_or("Untitled");
            let title = NSString::new(title);
            let () = msg_send![window, setTitle: title.raw];
            let () = msg_send![window, makeKeyAndOrderFront: nil];

            // setup view
            let ns_view: *mut Object = msg_send![self.application.view_delegate_class, alloc];

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

            // setup window delegate
            let window_delegate: *mut Object =
                msg_send![self.application.window_delegate_class, new];

            (*window_delegate).set_ivar("window_data", window);
            let () = msg_send![window, setDelegate: window_delegate];
            let () = msg_send![window, setContentView: ns_view];
            let () = msg_send![window, makeFirstResponder: ns_view];

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
        EventLoop {}
    }

    pub fn request_frame(&mut self) {
        unsafe {
            if let Some(app) = APP.as_mut() {
                app.frame_requested = true;
                self::poke_run_loop();
            }
        }
    }
}

fn poke_run_loop() {
    unsafe {
        if let Some(app) = APP.as_mut() {
            CFRunLoopSourceSignal(app.run_loop_custom_event_source);
            let run_loop = CFRunLoopGetMain();
            CFRunLoopWakeUp(run_loop);
        }
    }
}

pub struct EventLoop {}
impl EventLoop {
    pub fn run<T>(&self, callback: T)
    where
        T: 'static + FnMut(crate::Event),
    {
        println!("Running");
        unsafe {
            PROGRAM_CALLBACK = Some(Box::new(callback));
            if let Some(app) = APP.as_mut() {
                let () = msg_send![app.app, run];
            }
        }
    }
}
