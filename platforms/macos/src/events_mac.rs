use super::apple::*;
use super::application_mac::{get_window_data, APPLICATION_DATA};
use crate::{Event, Key, MouseButton, WindowId};
use std::ffi::c_void;

// ------------------------ Window Events --------------------------
extern "C" fn window_did_move(_this: &Object, _sel: Sel, ns_notification: *mut Object) {
    unsafe {
        let window: *const Object = msg_send![ns_notification, object];

        // Get backing scale to adjust for DPI.
        let backing_scale: CGFloat = msg_send![window, backingScaleFactor];
        let frame: CGRect = msg_send![window, frame];
        let screen: *const Object = msg_send![window, screen];
        let screen_frame: CGRect = msg_send![screen, frame];

        self::submit_event(crate::Event::WindowMoved {
            x: (frame.origin.x * backing_scale) as u32,
            y: ((screen_frame.size.height - frame.origin.y) * backing_scale) as u32, // Flip y coordinate because 0,0 is bottom left on Mac
            window_id: WindowId::new(window as *mut c_void),
        });
    }
}
extern "C" fn window_did_miniaturize(_this: &Object, _sel: Sel, ns_notification: *mut Object) {
    let window: *mut c_void = unsafe { msg_send![ns_notification, object] };
    self::submit_event(Event::WindowMinimized {
        window_id: WindowId::new(window),
    });
}

extern "C" fn window_did_deminiaturize(_this: &Object, _sel: Sel, ns_notification: *mut Object) {
    let window: *mut c_void = unsafe { msg_send![ns_notification, object] };
    self::submit_event(Event::WindowRestored {
        window_id: WindowId::new(window),
    });
}

extern "C" fn window_did_enter_fullscreen(_this: &Object, _sel: Sel, ns_notification: *mut Object) {
    let window: *mut c_void = unsafe { msg_send![ns_notification, object] };
    self::submit_event(Event::WindowFullscreened {
        window_id: WindowId::new(window),
    });
}
extern "C" fn window_did_exit_fullscreen(_this: &Object, _sel: Sel, ns_notification: *mut Object) {
    let window: *mut c_void = unsafe { msg_send![ns_notification, object] };
    self::submit_event(Event::WindowRestored {
        window_id: WindowId::new(window),
    });
}

extern "C" fn window_will_start_live_resize(
    _this: &Object,
    _sel: Sel,
    ns_notification: *mut Object,
) {
    let window: *mut c_void = unsafe { msg_send![ns_notification, object] };
    self::submit_event(crate::Event::WindowStartResize {
        window_id: WindowId::new(window),
    });
}

extern "C" fn window_did_end_live_resize(_this: &Object, _sel: Sel, ns_notification: *mut Object) {
    let window: *mut c_void = unsafe { msg_send![ns_notification, object] };
    self::submit_event(crate::Event::WindowEndResize {
        window_id: WindowId::new(window),
    });
}

extern "C" fn window_did_resize(_this: &Object, _sel: Sel, ns_notification: *mut Object) {
    unsafe {
        let window: *mut Object = msg_send![ns_notification, object];
        let view: *mut Object = msg_send![window, contentView];

        let backing_scale: CGFloat = msg_send![window, backingScaleFactor];
        let frame: CGRect = msg_send![view, frame];
        self::submit_event(crate::Event::WindowResized {
            width: (frame.size.width * backing_scale) as u32,
            height: (frame.size.height * backing_scale) as u32,
            window_id: WindowId::new(window as *mut c_void),
        });
    }
}

extern "C" fn window_did_become_key(_this: &Object, _sel: Sel, ns_notification: *mut Object) {
    let window: *mut c_void = unsafe { msg_send![ns_notification, object] };
    self::submit_event(crate::Event::WindowGainedFocus {
        window_id: WindowId::new(window),
    });
}

extern "C" fn window_did_resign_key(_this: &Object, _sel: Sel, ns_notification: *mut Object) {
    let window: *mut c_void = unsafe { msg_send![ns_notification, object] };
    self::submit_event(crate::Event::WindowLostFocus {
        window_id: WindowId::new(window),
    });
}

extern "C" fn window_should_close(_this: &Object, _sel: Sel, sender: *mut Object) -> BOOL {
    self::submit_event(crate::Event::WindowCloseRequested {
        window_id: WindowId::new(sender as *mut c_void),
    });
    NO // No because the program must drop its handle to close the window.
}

extern "C" fn window_did_change_backing_properties(_this: &Object, _sel: Sel, _event: *mut Object) {
    // Color space changes need to be detected here.
    // Info about how to check the old color space:
    // https://developer.apple.com/documentation/appkit/nswindowdelegate/1419517-windowdidchangebackingproperties

    // unsafe {
    // let window_data = get_window_data(this);
    // let backing_scale: CGFloat = msg_send![window_data.ns_window, backingScaleFactor];
    // let new_scale: CGFloat = msg_send![this, backingScaleFactor];

    // The color space could have changed, not the backing scale.
    // So check here to make sure the scale has actually changed.
    // However the check doesn't matter as no event is sent (yet!)
    /*
    if old_scale != new_scale {
        window_data.backing_scale = new_scale;
    }
    */
    // }
}

pub fn add_window_events_to_decl(decl: &mut ClassDecl) {
    unsafe {
        decl.add_method(
            sel!(windowShouldClose:),
            window_should_close as extern "C" fn(&Object, Sel, *mut Object) -> BOOL,
        );
        decl.add_method(
            sel!(windowDidMiniaturize:),
            window_did_miniaturize as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(windowDidDeminiaturize:),
            window_did_deminiaturize as extern "C" fn(&Object, Sel, *mut Object),
        );

        decl.add_method(
            sel!(windowDidEnterFullScreen:),
            window_did_enter_fullscreen as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(windowDidExitFullScreen:),
            window_did_exit_fullscreen as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(windowDidMove:),
            window_did_move as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(windowDidResize:),
            window_did_resize as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(windowWillStartLiveResize:),
            window_will_start_live_resize as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(windowDidEndLiveResize:),
            window_did_end_live_resize as extern "C" fn(&Object, Sel, *mut Object),
        );

        decl.add_method(
            sel!(windowDidChangeBackingProperties:),
            window_did_change_backing_properties as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(windowDidBecomeKey:),
            window_did_become_key as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(windowDidResignKey:),
            window_did_resign_key as extern "C" fn(&Object, Sel, *mut Object),
        );
    }
}

// ------------------------ End Window Events --------------------------
// ------------------------ Application Events --------------------------

extern "C" fn application_should_terminate_after_last_window_closed(
    _this: &Object,
    _sel: Sel,
    _sender: *mut Object,
) -> BOOL {
    NO // Do not close when all windows close.
}

// https://developer.apple.com/documentation/appkit/nsapplicationdelegate/1428642-applicationshouldterminate?language=objc
extern "C" fn application_should_terminate(
    _this: &Object,
    _sel: Sel,
    _sender: *mut Object,
) -> NSUInteger {
    if APPLICATION_DATA.with(|d| d.borrow().actually_terminate) {
        NSTerminateNow
    } else {
        self::submit_event(Event::QuitRequested);
        NSTerminateCancel
    }
}

extern "C" fn application_will_terminate(_this: &Object, _sel: Sel, _application: *mut Object) {
    self::submit_event(Event::Quit {});
}

pub fn add_application_events_to_decl(decl: &mut ClassDecl) {
    unsafe {
        decl.add_method(
            sel!(applicationShouldTerminateAfterLastWindowClosed:),
            application_should_terminate_after_last_window_closed
                as extern "C" fn(&Object, Sel, *mut Object) -> BOOL,
        );
        decl.add_method(
            sel!(applicationShouldTerminate:),
            application_should_terminate as extern "C" fn(&Object, Sel, *mut Object) -> NSUInteger,
        );
        decl.add_method(
            sel!(applicationWillTerminate:),
            application_will_terminate as extern "C" fn(&Object, Sel, *mut Object),
        );
    }
}
// ------------------------ End Application Events --------------------------

// ------------------------ View Events --------------------------
extern "C" fn draw_rect(this: &Object, _sel: Sel, _rect: CGRect) {
    let window: *mut Object = unsafe { msg_send![this, window] };
    kapp_platform_common::redraw_manager::draw(WindowId::new(window as *mut c_void));
}

extern "C" fn key_down(_this: &Object, _sel: Sel, event: *mut Object) {
    unsafe {
        let key_code = msg_send![event, keyCode];
        let repeat: bool = msg_send![event, isARepeat];
        let key = super::keys_mac::virtual_keycode_to_key(key_code);
        let event = if repeat {
            crate::Event::KeyRepeat {
                key,
                timestamp: get_timestamp(event),
            }
        } else {
            crate::Event::KeyDown {
                key,
                timestamp: get_timestamp(event),
            }
        };
        self::submit_event(event);
    }
}

extern "C" fn key_up(_this: &Object, _sel: Sel, event: *mut Object) {
    unsafe {
        let key_code = msg_send![event, keyCode];
        self::submit_event(crate::Event::KeyUp {
            key: super::keys_mac::virtual_keycode_to_key(key_code),
            timestamp: get_timestamp(event),
        });
    }
}

// https://developer.apple.com/documentation/appkit/nsresponder/1527647-flagschanged?language=objc
// This should be changed to keep track of the modifier state and only update if they were previously pressed.
// Caps lock keyup events are only registered when the key switches to an off state.
extern "C" fn flags_changed(_this: &Object, _sel: Sel, event: *mut Object) {
    fn get_modifier_state(modifier_flags: u64) -> [bool; 9] {
        [
            modifier_flags & NSEventModifierFlagCapsLock == NSEventModifierFlagCapsLock,
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
    const KEYS: [Key; 9] = [
        Key::CapsLock,
        Key::LeftShift,
        Key::RightShift,
        Key::LeftControl,
        Key::RightControl,
        Key::LeftAlt,
        Key::RightAlt,
        Key::Meta,
        Key::Meta,
    ];

    let modifier_flags_old = APPLICATION_DATA.with(|d| d.borrow().modifier_flags);

    let modifier_flags_new: NSUInteger = unsafe { msg_send![event, modifierFlags] };

    let flag_state_old = get_modifier_state(modifier_flags_old);
    let flag_state_new = get_modifier_state(modifier_flags_new);

    for i in 0..8 {
        if !flag_state_old[i] && flag_state_new[i] {
            self::submit_event(crate::Event::KeyDown {
                key: KEYS[i],
                timestamp: get_timestamp(event),
            })
        }

        if flag_state_old[i] && !flag_state_new[i] {
            self::submit_event(crate::Event::KeyUp {
                key: KEYS[i],
                timestamp: get_timestamp(event),
            })
        }
    }

    APPLICATION_DATA.with(|d| {
        d.borrow_mut().modifier_flags = modifier_flags_new;
    });
}

extern "C" fn mouse_moved(this: &Object, _sel: Sel, event: *mut Object) {
    let (x, y) = get_mouse_position(this, event);
    self::submit_event(crate::Event::MouseMoved {
        x: x as f32,
        y: y as f32,
        timestamp: get_timestamp(event),
    });
}

fn get_mouse_position(this: &Object, event: *mut Object) -> (f32, f32) {
    unsafe {
        let window_data = get_window_data(this);
        let backing_scale: CGFloat = msg_send![window_data.ns_window, backingScaleFactor];

        let window_point: NSPoint = msg_send![event, locationInWindow];
        let frame: CGRect = msg_send![window_data.ns_view, frame];

        let x = window_point.x * backing_scale;
        let y = (frame.size.height - window_point.y) * backing_scale; // Flip y coordinate because y is 0,0 on Mac.
        (x as f32, y as f32)
    }
}

extern "C" fn mouse_down(this: &Object, _sel: Sel, event: *mut Object) {
    let (x, y) = get_mouse_position(this, event);
    self::submit_event(crate::Event::MouseButtonDown {
        x,
        y,
        button: MouseButton::Left,
        timestamp: get_timestamp(event),
    });
}

extern "C" fn mouse_up(this: &Object, _sel: Sel, event: *mut Object) {
    let (x, y) = get_mouse_position(this, event);
    self::submit_event(crate::Event::MouseButtonUp {
        x,
        y,
        button: MouseButton::Left,
        timestamp: get_timestamp(event),
    });
}

extern "C" fn right_mouse_down(this: &Object, _sel: Sel, event: *mut Object) {
    let (x, y) = get_mouse_position(this, event);

    self::submit_event(crate::Event::MouseButtonDown {
        x,
        y,
        button: MouseButton::Right,
        timestamp: get_timestamp(event),
    });
}

extern "C" fn right_mouse_up(this: &Object, _sel: Sel, event: *mut Object) {
    let (x, y) = get_mouse_position(this, event);

    self::submit_event(crate::Event::MouseButtonUp {
        x,
        y,
        button: MouseButton::Right,
        timestamp: get_timestamp(event),
    });
}

extern "C" fn other_mouse_down(this: &Object, _sel: Sel, event: *mut Object) {
    let (x, y) = get_mouse_position(this, event);

    let number: NSInteger = unsafe { msg_send![event, buttonNumber] };
    let button = match number {
        // Are these correct?
        4 => MouseButton::Middle,
        8 => MouseButton::Extra1,
        16 => MouseButton::Extra2,
        _ => MouseButton::Unknown,
    };
    self::submit_event(crate::Event::MouseButtonDown {
        x,
        y,
        button,
        timestamp: get_timestamp(event),
    });
}

extern "C" fn other_mouse_up(this: &Object, _sel: Sel, event: *mut Object) {
    let number: NSInteger = unsafe { msg_send![event, buttonNumber] };
    let button = match number {
        // Are these correct?
        4 => MouseButton::Middle,
        8 => MouseButton::Extra1,
        16 => MouseButton::Extra2,
        _ => MouseButton::Unknown,
    };

    let (x, y) = get_mouse_position(this, event);
    self::submit_event(crate::Event::MouseButtonUp {
        x,
        y,
        button,
        timestamp: get_timestamp(event),
    });
}

extern "C" fn mouse_dragged(this: &Object, _sel: Sel, event: *mut Object) {
    let (x, y) = get_mouse_position(this, event);
    self::submit_event(crate::Event::MouseMoved {
        x,
        y,
        timestamp: get_timestamp(event),
    });
}

extern "C" fn right_mouse_dragged(this: &Object, _sel: Sel, event: *mut Object) {
    let (x, y) = get_mouse_position(this, event);
    self::submit_event(crate::Event::MouseMoved {
        x,
        y,
        timestamp: get_timestamp(event),
    });
}

extern "C" fn other_mouse_dragged(this: &Object, _sel: Sel, event: *mut Object) {
    let (x, y) = get_mouse_position(this, event);
    self::submit_event(crate::Event::MouseMoved {
        x,
        y,
        timestamp: get_timestamp(event),
    });
}

// https://developer.apple.com/documentation/appkit/nsresponder/1534192-scrollwheel?language=objc
extern "C" fn scroll_wheel(_this: &Object, _sel: Sel, event: *mut Object) {
    let delta_x: CGFloat = unsafe { msg_send![event, scrollingDeltaX] };
    let delta_y: CGFloat = unsafe { msg_send![event, scrollingDeltaY] };

    self::submit_event(crate::Event::Scroll {
        delta_x: delta_x as f32,
        delta_y: delta_y as f32,
        timestamp: get_timestamp(event),
    });
}

extern "C" fn accepts_first_responder(_this: &Object, _sel: Sel) -> BOOL {
    YES
}

// https://developer.apple.com/documentation/appkit/nsresponder/1525862-magnifywithevent
extern "C" fn magnify_with_event(_this: &Object, _sel: Sel, event: *mut Object) {
    let magnification: CGFloat = unsafe { msg_send![event, magnification] };

    self::submit_event(crate::Event::PinchGesture {
        delta: magnification as f32,
        timestamp: get_timestamp(event),
    });
}

pub fn add_view_events_to_decl(decl: &mut ClassDecl) {
    unsafe {
        decl.add_method(
            sel!(magnifyWithEvent:),
            magnify_with_event as extern "C" fn(&Object, Sel, *mut Object),
        );

        decl.add_method(
            sel!(drawRect:),
            draw_rect as extern "C" fn(&Object, Sel, CGRect),
        );
        decl.add_method(
            sel!(acceptsFirstResponder),
            accepts_first_responder as extern "C" fn(&Object, Sel) -> BOOL,
        );
        decl.add_method(
            sel!(scrollWheel:),
            scroll_wheel as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(otherMouseDown:),
            other_mouse_down as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(otherMouseUp:),
            other_mouse_up as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(rightMouseDown:),
            right_mouse_down as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(rightMouseUp:),
            right_mouse_up as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(mouseDown:),
            mouse_down as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(mouseUp:),
            mouse_up as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(mouseMoved:),
            mouse_moved as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(mouseDragged:),
            mouse_dragged as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(rightMouseDragged:),
            right_mouse_dragged as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(otherMouseDragged:),
            other_mouse_dragged as extern "C" fn(&Object, Sel, *mut Object),
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

// ------------------------ End View Events --------------------------
pub fn submit_event(event: Event) {
    kapp_platform_common::event_receiver::send_event(event);
}

pub fn get_timestamp(event: *mut Object) -> std::time::Duration {
    let number: f64 = unsafe { msg_send![event, timestamp] };

    std::time::Duration::from_secs_f64(number)
}
