use super::apple::*;
use kapp_platform_common::{WindowId, WindowParameters};
use std::ffi::c_void;

// All of this data and the instances must be all be dropped together.
// Window and GLContext can hold a strong ref to this data, ns_window and ns_view will hold a raw pointer to this data.
// Because ns_window and ns_view will only be released only when this is dropped, the raw pointers should always be valid.
pub struct InnerWindowData {
    pub ns_window: *mut Object,
    pub ns_view: *mut Object, // Used later by GLContext.
    window_delegate: *mut Object,
    tracking_area: *mut Object,
    //pub backing_scale: f64, // On Mac this while likely be either 2.0 or 1.0
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
    window_parameters: &WindowParameters,
    window_class: *const objc::runtime::Class,
    view_class: *const objc::runtime::Class,
) -> Result<WindowId, ()> {
    unsafe {
        // The window width and height doesn't matter initially because it will
        // just be reset with another call once the backing scale is known.
        let rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(500.0, 500.0));

        let mut style =
            NSWindowStyleMaskTitled | NSWindowStyleMaskClosable | NSWindowStyleMaskMiniaturizable;

        if window_parameters.resizable {
            style |= NSWindowStyleMaskResizable;
        }

        if window_parameters.without_titlebar {
            style |= NSWindowStyleMaskFullSizeContentView
        }

        // This allocation will be released when the window is dropped.
        let ns_window: *mut Object = msg_send![class!(NSWindow), alloc];
        let () = msg_send![
            ns_window,
            initWithContentRect:rect.clone()
            styleMask:style
            backing:NSBackingStoreBuffered
            defer:NO
        ];

        if window_parameters.without_titlebar {
            let () = msg_send![ns_window, setTitlebarAppearsTransparent: 1];
            let () = msg_send![ns_window, setTitleVisibility: 1];
        }

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

        // Set minimum size
        if let Some((width, height)) = window_parameters.minimum_size {
            let () = msg_send![ns_window, setMinSize: NSSize::new((width as f64) / backing_scale, (height as f64) / backing_scale)];
        }

        // Set the window size
        // This should always be set
        if let Some((width, height)) = window_parameters.size {
            let () = msg_send![ns_window, setContentSize: NSSize::new((width as f64) / backing_scale, (height as f64) / backing_scale)];
        }

        let title = NSString::new(&window_parameters.title);
        let () = msg_send![ns_window, setTitle: title.raw];
        let () = msg_send![ns_window, makeKeyAndOrderFront: nil];

        // Setup window delegate that receives events.
        // This allocation will be released when the window is dropped.
        let window_delegate: *mut Object = msg_send![window_class, new];

        // Setup view
        // This allocation will be released when the window is dropped.
        let ns_view: *mut Object = msg_send![view_class, alloc];
        let () = msg_send![ns_view, initWithFrame: rect.clone()];

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

        Ok(WindowId::new(ns_window as *mut c_void))
    }
}
