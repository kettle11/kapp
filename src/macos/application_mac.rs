use super::apple::*;
use crate::Event;
use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

pub type ProgramCallback = dyn 'static + FnMut(Event);

pub static INSTANCE_DATA_IVAR_ID: &str = "instance_data";
static WINDOW_CLASS_NAME: &str = "KettlewinWindowClass";
static VIEW_CLASS_NAME: &str = "KettlewinViewClass";
static APPLICATION_CLASS_NAME: &str = "KettlewinApplicationClass";

#[derive(Clone)]
pub struct Window {
    pub id: WindowId,
    pub inner_window_data: Rc<RefCell<InnerWindowData>>, // this shouldn't be public
}

// All of this data and the instances must be all be dropped together.
// Window and GLContext can hold a strong ref to this data, ns_window and ns_view will hold a raw pointer to this data.
// Because ns_window and ns_view will only be released only when this is dropped, the raw pointers should always be valid.
pub struct InnerWindowData {
    pub ns_window: *mut Object,
    pub ns_view: *mut Object, // Used later by GLContext.
    window_delegate: *mut Object,
    tracking_area: *mut Object,

    pub application_data: Rc<RefCell<ApplicationData>>,
    pub backing_scale: f64, // On Mac this while likely be either 2.0 or 1.0
    pub window_state: WindowState,
}

impl Drop for InnerWindowData {
    fn drop(&mut self) {
        unsafe {
            let () = msg_send![self.ns_window, close];
            let () = msg_send![self.window_delegate, release];
            let () = msg_send![self.ns_view, release];
            let () = msg_send![self.tracking_area, release];
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct WindowId {
    // This should not be public
    pub ns_window: *mut Object, // Just use the window pointer as the ID, it's unique.
}

impl std::fmt::Debug for WindowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            // Retrieve the window title and use that to make more legible events
            let title: *mut Object = msg_send![self.ns_window, title];
            let title: *const i8 = msg_send![title, UTF8String];
            let title = std::ffi::CStr::from_ptr(title);
            f.write_fmt(format_args!(
                "[Title: {:?}, Pointer: {:?}]",
                title, self.ns_window
            ))
        }
    }
}

// Not exposed outside the crate
pub enum WindowState {
    Minimized,
    Windowed, // The typical state a window is in.
    Fullscreen,
}

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

            // This allocation will be released when the window is dropped.
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
            let backing_scale: CGFloat = msg_send![ns_window, backingScaleFactor];

            // Setup window delegate that receives events.
            // This allocation will be released when the window is dropped.
            let window_delegate: *mut Object = msg_send![self.application.window_class, new];
            // Heap allocate a data structure for the window.
            // This allocation will be released when the window is dropped.

            // let window_instance_data =
            //    Box::leak(window_instance_data_box) as *mut WindowInstanceData as *mut c_void;

            // setup view
            // This allocation will be released when the window is dropped.
            let ns_view: *mut Object = msg_send![self.application.view_class, alloc];

            // Apparently this defaults to YES even without this call
            let () = msg_send![ns_view, setWantsBestResolutionOpenGLSurface: YES];

            // Setup a tracking area to receive mouse events within
            // This allocation will be released when the window is dropped.
            let tracking_area: *mut Object = msg_send![class!(NSTrackingArea), alloc];
            let () = msg_send![
                tracking_area,
                initWithRect: rect
                options: NSTrackingMouseEnteredAndExited | NSTrackingMouseMoved | NSTrackingActiveInKeyWindow | NSTrackingInVisibleRect
                owner: ns_view
                userInfo:nil];
            let () = msg_send![ns_view, addTrackingArea: tracking_area];

            let () = msg_send![ns_window, setDelegate: window_delegate];
            let () = msg_send![ns_window, setContentView: ns_view];
            let () = msg_send![ns_window, makeFirstResponder: ns_view];

            let inner_window_data = Rc::new(RefCell::new(InnerWindowData {
                ns_window,
                ns_view,
                window_delegate,
                tracking_area,
                application_data: Rc::clone(&self.application.application_data),
                backing_scale,
                window_state: WindowState::Windowed,
            }));

            // Give weak references to the window data to the window_delegate and ns_view_delegate.
            (*window_delegate).set_ivar(
                INSTANCE_DATA_IVAR_ID,
                inner_window_data.as_ptr() as *mut c_void,
            );
            (*ns_view).set_ivar(
                INSTANCE_DATA_IVAR_ID,
                inner_window_data.as_ptr() as *mut c_void,
            );

            let window = Window {
                id: WindowId { ns_window },
                inner_window_data,
            };
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

    pub fn quit(&self) {
        let ns_application = self.application_data.borrow().ns_application;
        unsafe {
            let () = msg_send![ns_application, terminate: nil];
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

impl Window {
    pub fn minimize(&self) {
        let inner_window_data = self.inner_window_data.borrow();
        unsafe {
            let () = msg_send![inner_window_data.ns_window, miniaturize: nil];
        }
    }

    pub fn maximize(&self) {
        // Does nothing for now
        // MacOS only has the notion of 'fullscreen' not of maximize.
    }

    /// Returns the window from a minimized or maximized state.
    pub fn restore(&self) {
        unsafe {
            let inner_window_data = self.inner_window_data.borrow();

            match inner_window_data.window_state {
                WindowState::Minimized => {
                    let () = msg_send![inner_window_data.ns_window, deminiaturize: nil];
                }
                WindowState::Fullscreen => {
                    let () = msg_send![inner_window_data.ns_window, toggleFullScreen: nil];
                }
                _ => {}
            }
        }
    }

    pub fn fullscreen(&self) {
        let inner_window_data = self.inner_window_data.borrow();

        unsafe {
            let () = msg_send![inner_window_data.ns_window, toggleFullScreen: nil];
        }
    }

    /// Set the lower left corner of the window.
    pub fn set_position(&self, x: u32, y: u32) {
        unsafe {
            let inner_window_data = self.inner_window_data.borrow();

            // Accounts for scale factor
            let backing_scale = inner_window_data.backing_scale;

            let () =
                msg_send![inner_window_data.ns_window, setFrameOrigin: NSPoint::new((x as f64) / backing_scale, (y as f64) / backing_scale)];
        }
    }

    /// Set the window's width and height, excluding the titlebar
    pub fn set_size(&self, width: u32, height: u32) {
        unsafe {
            let inner_window_data = self.inner_window_data.borrow();

            // Accounts for scale factor
            let backing_scale = inner_window_data.backing_scale;

            match inner_window_data.window_state {
                WindowState::Fullscreen => {} // Don't resize the window if fullscreen.
                _ => {
                    let () =
                        msg_send![inner_window_data.ns_window, setContentSize: NSSize::new((width as f64) / backing_scale, (height as f64) / backing_scale)];
                }
            }
        }
    }
}