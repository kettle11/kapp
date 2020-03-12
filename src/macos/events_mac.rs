use super::apple::*;
use crate::Button;
// ------------------------ Window Events --------------------------
extern "C" fn window_moved(_this: &Object, _sel: Sel, _event: *mut Object) {}
extern "C" fn window_did_resize(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = super::application_mac::get_window_instance_data(this);

    unsafe {
        let frame: CGRect = msg_send![(*window_data).ns_window, frame];
        self::produce_event_from_window(
            this,
            crate::Event::ResizedWindow {
                width: frame.size.width as u32,
                height: frame.size.height as u32,
            },
        );
    }
}
// ------------------------ End Window Events --------------------------

// ------------------------ View Events --------------------------
extern "C" fn key_down(this: &Object, _sel: Sel, event: *mut Object) {
    unsafe {
        let key_code = msg_send![event, keyCode];
        self::produce_event_from_view(
            this,
            crate::Event::ButtonDown {
                button: super::keys_mac::virtual_keycode_to_key(key_code),
                scancode: 0,
            },
        );
    }
}

extern "C" fn key_up(this: &Object, _sel: Sel, event: *mut Object) {
    unsafe {
        let key_code = msg_send![event, keyCode];
        self::produce_event_from_view(
            this,
            crate::Event::ButtonUp {
                button: super::keys_mac::virtual_keycode_to_key(key_code),
                scancode: 0,
            },
        );
    }
}

// https://developer.apple.com/documentation/appkit/nsresponder/1527647-flagschanged?language=objc
// This should be changed to keep track of the modifier state and only update if they were previously pressed.
extern "C" fn flags_changed(this: &Object, _sel: Sel, event: *mut Object) {
    fn get_modifier_state(modifier_flags: u64) -> [bool; 8] {
        [
            modifier_flags & NX_DEVICELSHIFTKEYMASK == NX_DEVICELSHIFTKEYMASK,
            modifier_flags & NX_DEVICERSHIFTKEYMASK == NX_DEVICERSHIFTKEYMASK,
            modifier_flags & NX_DEVICELCTLKEYMASK == NX_DEVICELCTLKEYMASK,
            modifier_flags & NX_DEVICERCTLKEYMASK == NX_DEVICERCTLKEYMASK,
            modifier_flags & NX_DEVICELALTKEYMASK == NX_DEVICELALTKEYMASK,
            modifier_flags & NX_DEVICERALTKEYMASK == NX_DEVICERALTKEYMASK,
            modifier_flags & NX_DEVICELCMDKEYMASK == NX_DEVICELCMDKEYMASK,
            modifier_flags & NX_DEVICERCMDKEYMASK == NX_DEVICERCMDKEYMASK,
        ]
    }

    // These correspond to the modifier flag array.
    const BUTTONS: [Button; 8] = [
        Button::LeftShift,
        Button::RightShift,
        Button::LeftControl,
        Button::RightControl,
        Button::LeftAlt,
        Button::RightAlt,
        Button::Meta,
        Button::Meta,
    ];

    let window_data = super::application_mac::get_window_instance_data(this);
    let modifier_flags_old = unsafe { (*window_data).application_data.borrow().modifier_flags };
    let modifier_flags_new: NSUInteger = unsafe { msg_send![event, modifierFlags] };

    let flag_state_old = get_modifier_state(modifier_flags_old);
    let flag_state_new = get_modifier_state(modifier_flags_new);

    for i in 0..8 {
        if !flag_state_old[i] && flag_state_new[i] {
            self::produce_event_from_window(
                this,
                crate::Event::ButtonDown {
                    button: BUTTONS[i],
                    scancode: 0,
                },
            )
        }

        if flag_state_old[i] && !flag_state_new[i] {
            self::produce_event_from_window(
                this,
                crate::Event::ButtonUp {
                    button: BUTTONS[i],
                    scancode: 0,
                },
            )
        }
    }

    unsafe {
        (*window_data).application_data.borrow_mut().modifier_flags = modifier_flags_new;
    }
}

extern "C" fn mouse_moved(this: &Object, _sel: Sel, event: *mut Object) {
    unsafe {
        // The following code snippet is taken from winit.
        // We have to do this to have access to the `NSView` trait...
        let view: *mut Object = this as *const _ as *mut _;

        let window_point: NSPoint = msg_send![event, locationInWindow];
        let view_point: NSPoint = msg_send![view, convertPoint: window_point fromView: nil];
        let view_rect: NSRect = msg_send![this, frame];

        if view_point.x.is_sign_negative()
            || view_point.y.is_sign_negative()
            || view_point.x > view_rect.size.width
            || view_point.y > view_rect.size.height
        {
            // Point is outside of the client area (view)
            return;
        }

        let x = view_point.x;
        let y = view_rect.size.height - view_point.y;

        self::produce_event_from_view(
            this,
            crate::Event::MouseMoved {
                x: x as f32,
                y: y as f32,
            },
        );
    }
}
// ------------------------ End View Events --------------------------

pub fn produce_event_from_window(this: &Object, event: crate::Event) {
    let window_data = super::application_mac::get_window_instance_data(this);
    unsafe {
        // This is a little awkward, but the application_data cannot be borrowed
        // while the program_callback is called as it may call functions that borrow application_data
        let mut program_callback = (*window_data)
            .application_data
            .borrow_mut()
            .program_callback
            .take();
        if let Some(callback) = program_callback.as_mut() {
            callback(event)
        }
        (*window_data)
            .application_data
            .borrow_mut()
            .program_callback = program_callback;
    }
}

pub fn produce_event_from_view(this: &Object, event: crate::Event) {
    // First get the view's window
    unsafe {
        let view_data = super::application_mac::get_view_instance_data(this);

        // This is a little awkward, but the application_data cannot be borrowed
        // while the program_callback is called as it may call functions that borrow application_data
        let mut program_callback = (*view_data)
            .application_data
            .borrow_mut()
            .program_callback
            .take();
        if let Some(callback) = program_callback.as_mut() {
            callback(event)
        }
        (*view_data).application_data.borrow_mut().program_callback = program_callback;
    }
}

pub fn add_window_events_to_decl(decl: &mut ClassDecl) {
    unsafe {
        decl.add_method(
            sel!(windowDidMove:),
            window_moved as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(windowDidResize:),
            window_did_resize as extern "C" fn(&Object, Sel, *mut Object),
        );
    }
}

pub fn add_view_events_to_decl(decl: &mut ClassDecl) {
    unsafe {
        decl.add_method(
            sel!(mouseMoved:),
            mouse_moved as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(keyDown:),
            key_down as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(keyUp:),
            key_up as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(flagsChanged:),
            flags_changed as extern "C" fn(&Object, Sel, *mut Object),
        );
    }
}
