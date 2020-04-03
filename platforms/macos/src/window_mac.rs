use super::apple::*;
use super::application_mac::{ApplicationData, INSTANCE_DATA_IVAR_ID};
use crate::WindowId;
use std::ffi::c_void;

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

    //pub backing_scale: f64, // On Mac this while likely be either 2.0 or 1.0
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

pub fn build(
    window_parameters: &crate::WindowParameters,
    window_class: *const objc::runtime::Class,
    view_class: *const objc::runtime::Class,
) -> Result<WindowId, ()> {
    unsafe {
        let (width, height) = window_parameters
            .dimensions
            .map_or((600., 600.), |(width, height)| {
                (width as f64, height as f64)
            });
        let rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(width, height));

        let mut style =
            NSWindowStyleMaskTitled | NSWindowStyleMaskClosable | NSWindowStyleMaskMiniaturizable;

        if window_parameters.resizable {
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

        if let Some(position) = window_parameters.position {
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

        let title = window_parameters.title.as_ref().map_or("Untitled", |s| &s);
        let title = NSString::new(&title);
        let () = msg_send![ns_window, setTitle: title.raw];
        let () = msg_send![ns_window, makeKeyAndOrderFront: nil];

        // Setup window delegate that receives events.
        // This allocation will be released when the window is dropped.
        let window_delegate: *mut Object = msg_send![window_class, new];

        // Setup view
        // This allocation will be released when the window is dropped.
        let ns_view: *mut Object = msg_send![view_class, alloc];

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

        let inner_window_data = Box::new(InnerWindowData {
            ns_window,
            ns_view,
            window_delegate,
            tracking_area,
            // backing_scale,
            window_state: WindowState::Windowed,
        });

        // This is never dropped, but should be.
        let inner_window_data_ptr = Box::into_raw(inner_window_data);

        // Give weak references to the window data to the window_delegate and ns_view_delegate.
        (*window_delegate).set_ivar(INSTANCE_DATA_IVAR_ID, inner_window_data_ptr as *mut c_void);
        (*ns_view).set_ivar(INSTANCE_DATA_IVAR_ID, inner_window_data_ptr as *mut c_void);

        Ok(WindowId::new(ns_window as *mut c_void))
    }
}
