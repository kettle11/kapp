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

            // Stuff taken from Winit to setup a loopobserver
            extern "C" fn control_flow_end_handler(
                _: CFRunLoopObserverRef,
                _: CFRunLoopActivity,
                _: *mut std::ffi::c_void,
            ) {
                super::events_mac::produce_event(Event::Draw);
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

            // This event is empty because it's only used continuously wakeup the main thread.
            extern "C" fn wakeup_main_loop(
                _timer: CFRunLoopTimerRef,
                _info: *mut std::ffi::c_void,
            ) {
            }
            let timer = CFRunLoopTimerCreate(
                std::ptr::null_mut(),
                0.,
                0.000_000_1,
                0,
                0,
                wakeup_main_loop,
                std::ptr::null_mut(),
            );
            CFRunLoopAddTimer(CFRunLoopGetMain(), timer, kCFRunLoopCommonModes);

            let app = Application {
                app,
                window_delegate_class: window_delegate_declaration(),
                view_delegate_class: view_delegate_declaration(),
            };
            APP = Some(Box::new(app.clone()));
            Ok(app)
        }
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
            let (x, y) = self
                .position
                .map_or((0., 0.), |(x, y)| (x as f64, y as f64));

            let (width, height) = self.dimensions.map_or((600., 600.), |(width, height)| {
                (width as f64, height as f64)
            });
            let rect = NSRect::new(NSPoint::new(x, y), NSSize::new(width, height));

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

            let () = msg_send![window, cascadeTopLeftFromPoint:NSPoint::new(20., 20.)];

            let () = msg_send![window, center];
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
