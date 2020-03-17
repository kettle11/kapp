use super::apple::*;
use super::application_mac::{Application, ApplicationData, INSTANCE_DATA_IVAR_ID};
use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

#[derive(Clone)]
pub struct Window {
    pub id: WindowId,
    pub inner_window_data: Rc<RefCell<InnerWindowData>>, // this shouldn't be public
}

// Not exposed outside the crate
pub enum WindowState {
    Minimized,
    Windowed, // The typical state a window is in.
    Fullscreen,
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
    ns_window: *mut Object, // Just use the window pointer as the ID, it's unique.
}

impl WindowId {
    pub fn new(ns_window: *mut Object) -> Self {
        Self { ns_window }
    }
}

// Typically WindowId is unsafe to send, but the ns_window field is only used
// as a unique id so it's ok.
unsafe impl Send for WindowId {}

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

pub struct WindowBuilder<'a> {
    application: &'a Application,
    pub position: Option<(u32, u32)>,
    pub dimensions: Option<(u32, u32)>,
    pub resizable: bool,
    pub title: Option<&'a str>,
}

impl<'a> WindowBuilder<'a> {
    pub fn new(application: &'a Application) -> Self {
        Self {
            application,
            position: None,
            dimensions: None,
            resizable: true,
            title: None,
        }
    }
    
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
            let backing_scale: CGFloat = msg_send![ns_window, backingScaleFactor];

            if let Some(position) = self.position {
                let position = (
                    position.0 as f64 / backing_scale,
                    position.1 as f64 / backing_scale,
                );
                let () = msg_send![ns_window, cascadeTopLeftFromPoint:NSPoint::new(position.0 as f64, position.1 as f64)];
            } else {
                // Center the window if no position is specified.
                let () = msg_send![ns_window, center];
            }

            // Set the window size
            let () = msg_send![ns_window, setContentSize: NSSize::new((width as f64) / backing_scale, (height as f64) / backing_scale)];

            let title = self.title.unwrap_or("Untitled");
            let title = NSString::new(title);
            let () = msg_send![ns_window, setTitle: title.raw];
            let () = msg_send![ns_window, makeKeyAndOrderFront: nil];

            // Setup window delegate that receives events.
            // This allocation will be released when the window is dropped.
            let window_delegate: *mut Object = msg_send![self.application.get_window_class(), new];
            // Heap allocate a data structure for the window.
            // This allocation will be released when the window is dropped.

            // let window_instance_data =
            //    Box::leak(window_instance_data_box) as *mut WindowInstanceData as *mut c_void;

            // setup view
            // This allocation will be released when the window is dropped.
            let ns_view: *mut Object = msg_send![self.application.get_view_class(), alloc];

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
            let () = msg_send![ns_view, setAcceptsTouchEvents: YES];

            let () = msg_send![ns_window, setDelegate: window_delegate];
            let () = msg_send![ns_window, setContentView: ns_view];
            let () = msg_send![ns_window, makeFirstResponder: ns_view];

            let inner_window_data = Rc::new(RefCell::new(InnerWindowData {
                ns_window,
                ns_view,
                window_delegate,
                tracking_area,
                application_data: Rc::clone(&self.application.get_application_data()),
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

    pub fn backing_scale(&self) -> f64 {
        let inner_window_data = self.inner_window_data.borrow();
        inner_window_data.backing_scale
    }
}
