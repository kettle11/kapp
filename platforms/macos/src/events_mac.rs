use super::apple::*;
use super::application_mac::{get_window_data, APPLICATION_DATA};
use super::window_mac::WindowState;
use crate::{Event, Key, MouseButton, WindowId};
use std::ffi::c_void;

// ------------------------ Window Events --------------------------

extern "C" fn window_did_move(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);
    unsafe {
        let backing_scale: CGFloat = msg_send![window_data.ns_window, backingScaleFactor];
        let frame: CGRect = msg_send![window_data.ns_window, frame];
        self::submit_event(crate::Event::WindowMoved {
            x: (frame.origin.x * backing_scale) as u32,
            y: (frame.origin.y * backing_scale) as u32,
            window_id: WindowId::new(window_data.ns_window as *mut c_void),
        });
    }
}
extern "C" fn window_did_miniaturize(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);
    window_data.window_state = WindowState::Minimized;
    self::submit_event(Event::WindowMinimized {
        window_id: WindowId::new(window_data.ns_window as *mut c_void),
    });
}

extern "C" fn window_did_deminiaturize(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);
    window_data.window_state = WindowState::Windowed; // Is this correct if the window immediately fullscreens?
    self::submit_event(Event::WindowRestored {
        window_id: WindowId::new(window_data.ns_window as *mut c_void),
    });
}

extern "C" fn window_did_enter_fullscreen(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);
    window_data.window_state = WindowState::Fullscreen;
    self::submit_event(Event::WindowFullscreened {
        window_id: WindowId::new(window_data.ns_window as *mut c_void),
    });
}
extern "C" fn window_did_exit_fullscreen(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);
    window_data.window_state = WindowState::Windowed; // Is this correct if the window immediately minimizes?
    self::submit_event(Event::WindowRestored {
        window_id: WindowId::new(window_data.ns_window as *mut c_void),
    });
}

extern "C" fn window_will_start_live_resize(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);

    self::submit_event(crate::Event::WindowStartResize {
        window_id: WindowId::new(window_data.ns_window as *mut c_void),
    });
}

extern "C" fn window_did_end_live_resize(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);

    self::submit_event(crate::Event::WindowEndResize {
        window_id: WindowId::new(window_data.ns_window as *mut c_void),
    });
}

extern "C" fn window_did_resize(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);

    unsafe {
        let backing_scale: CGFloat = msg_send![window_data.ns_window, backingScaleFactor];
        let frame: CGRect = msg_send![window_data.ns_window, frame];
        self::submit_event(crate::Event::WindowResized {
            width: (frame.size.width * backing_scale) as u32,
            height: (frame.size.height * backing_scale) as u32,
            window_id: WindowId::new(window_data.ns_window as *mut c_void),
        });
    }
}

extern "C" fn window_did_change_backing_properties(_this: &Object, _sel: Sel, _event: *mut Object) {
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

extern "C" fn window_did_become_key(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);
    self::submit_event(crate::Event::WindowGainedFocus {
        window_id: WindowId::new(window_data.ns_window as *mut c_void),
    });
}

extern "C" fn window_did_resign_key(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);
    self::submit_event(crate::Event::WindowLostFocus {
        window_id: WindowId::new(window_data.ns_window as *mut c_void),
    });
}

extern "C" fn window_should_close(this: &Object, _sel: Sel, _event: *mut Object) -> BOOL {
    let window_data = get_window_data(this);
    self::submit_event(crate::Event::WindowCloseRequested {
        window_id: WindowId::new(window_data.ns_window as *mut c_void),
    });
    NO // No because the program must drop its handle to close the window.
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
    _event: *mut Object,
) -> BOOL {
    YES // Close when all windows close.
}

// https://developer.apple.com/documentation/appkit/nsapplicationdelegate/1428642-applicationshouldterminate?language=objc
extern "C" fn application_should_terminate(
    _this: &Object,
    _sel: Sel,
    _event: *mut Object,
) -> NSUInteger {
    self::submit_event(Event::Quit);
    NSTerminateNow
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
    }
}
// ------------------------ End Application Events --------------------------

// ------------------------ View Events --------------------------
extern "C" fn key_down(_this: &Object, _sel: Sel, event: *mut Object) {
    unsafe {
        let key_code = msg_send![event, keyCode];
        let repeat: bool = msg_send![event, isARepeat];
        let key = super::keys_mac::virtual_keycode_to_key(key_code);
        let event = if repeat {
            crate::Event::KeyRepeat { key }
        } else {
            crate::Event::KeyDown { key }
        };
        self::submit_event(event);
    }
}

extern "C" fn key_up(_this: &Object, _sel: Sel, event: *mut Object) {
    unsafe {
        let key_code = msg_send![event, keyCode];
        self::submit_event(crate::Event::KeyUp {
            key: super::keys_mac::virtual_keycode_to_key(key_code),
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

    let modifier_flags_old = APPLICATION_DATA.with(|d| d.borrow().as_ref().unwrap().modifier_flags);

    let modifier_flags_new: NSUInteger = unsafe { msg_send![event, modifierFlags] };

    let flag_state_old = get_modifier_state(modifier_flags_old);
    let flag_state_new = get_modifier_state(modifier_flags_new);

    for i in 0..8 {
        if !flag_state_old[i] && flag_state_new[i] {
            self::submit_event(crate::Event::KeyDown { key: KEYS[i] })
        }

        if flag_state_old[i] && !flag_state_new[i] {
            self::submit_event(crate::Event::KeyUp { key: KEYS[i] })
        }
    }

    APPLICATION_DATA.with(|d| {
        d.borrow_mut().as_mut().unwrap().modifier_flags = modifier_flags_new;
    });
}

extern "C" fn mouse_moved(this: &Object, _sel: Sel, event: *mut Object) {
    unsafe {
        let window_data = get_window_data(this);
        let backing_scale: CGFloat = msg_send![window_data.ns_window, backingScaleFactor];

        let window_point: NSPoint = msg_send![event, locationInWindow];
        let x = window_point.x * backing_scale;
        let y = window_point.y * backing_scale; // Don't flip because 0 is bottom left on MacOS

        self::submit_event(crate::Event::MouseMoved {
            x: x as f32,
            y: y as f32,
        });
    }
}

extern "C" fn mouse_down(_this: &Object, _sel: Sel, _event: *mut Object) {
    self::submit_event(crate::Event::MouseButtonDown {
        button: MouseButton::Left,
    });
}

extern "C" fn mouse_up(_this: &Object, _sel: Sel, _event: *mut Object) {
    self::submit_event(crate::Event::MouseButtonUp {
        button: MouseButton::Left,
    });
}

extern "C" fn right_mouse_down(_this: &Object, _sel: Sel, _event: *mut Object) {
    self::submit_event(crate::Event::MouseButtonDown {
        button: MouseButton::Right,
    });
}

extern "C" fn right_mouse_up(_this: &Object, _sel: Sel, _event: *mut Object) {
    self::submit_event(crate::Event::MouseButtonUp {
        button: MouseButton::Right,
    });
}

extern "C" fn other_mouse_down(_this: &Object, _sel: Sel, event: *mut Object) {
    let number: NSInteger = unsafe { msg_send![event, buttonNumber] };
    let button = match number {
        // Are these correct?
        4 => MouseButton::Middle,
        8 => MouseButton::Extra1,
        16 => MouseButton::Extra2,
        _ => MouseButton::Unknown,
    };
    self::submit_event(crate::Event::MouseButtonDown { button });
}

extern "C" fn other_mouse_up(_this: &Object, _sel: Sel, event: *mut Object) {
    let number: NSInteger = unsafe { msg_send![event, buttonNumber] };
    let button = match number {
        // Are these correct?
        4 => MouseButton::Middle,
        8 => MouseButton::Extra1,
        16 => MouseButton::Extra2,
        _ => MouseButton::Unknown,
    };
    self::submit_event(crate::Event::MouseButtonUp { button });
}

// https://developer.apple.com/documentation/appkit/nsresponder/1534192-scrollwheel?language=objc
extern "C" fn scroll_wheel(_this: &Object, _sel: Sel, event: *mut Object) {
    let delta_y: CGFloat = unsafe { msg_send![event, scrollingDeltaY] };

    self::submit_event(crate::Event::ScrollWheel {
        delta: delta_y as f32,
    });
}

extern "C" fn accepts_first_responder(_this: &Object, _sel: Sel) -> BOOL {
    YES
}

// https://developer.apple.com/documentation/appkit/nsresponder/1531151-touchesbeganwithevent?language=objc
extern "C" fn touches_began_with_event(this: &Object, _sel: Sel, event: *mut Object) {
    unsafe {
        let touches: *mut Object =
            msg_send![event, touchesMatchingPhase:NSTouchPhaseBegan inView: this];

        let array: *mut Object = msg_send![touches, allObjects];
        let count: NSUInteger = msg_send![array, count];
        for i in 0..count {
            let touch: *mut Object = msg_send![array, objectAtIndex: i];
            let position: NSPoint = msg_send![touch, normalizedPosition];
            self::submit_event(crate::Event::TrackpadTouch {
                x: position.x as f32,
                y: position.y as f32,
            });
        }
    }
}

// https://developer.apple.com/documentation/appkit/nsresponder/1531151-touchesbeganwithevent?language=objc
extern "C" fn touches_moved_with_event(this: &Object, _sel: Sel, event: *mut Object) {
    unsafe {
        let touches: *mut Object =
            msg_send![event, touchesMatchingPhase:NSTouchPhaseMoved inView: this];

        let array: *mut Object = msg_send![touches, allObjects];
        let count: NSUInteger = msg_send![array, count];
        for i in 0..count {
            let touch: *mut Object = msg_send![array, objectAtIndex: i];
            let position: NSPoint = msg_send![touch, normalizedPosition];
            self::submit_event(crate::Event::TrackpadTouch {
                x: position.x as f32,
                y: position.y as f32,
            });
        }
    }
}

pub fn add_view_events_to_decl(decl: &mut ClassDecl) {
    unsafe {
        decl.add_method(
            sel!(touchesBeganWithEvent:),
            touches_began_with_event as extern "C" fn(&Object, Sel, *mut Object),
        );
        decl.add_method(
            sel!(touchesMovedWithEvent:),
            touches_moved_with_event as extern "C" fn(&Object, Sel, *mut Object),
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
    &mut APPLICATION_DATA.with(|d| {
        if let Some(callback) = d
            .borrow_mut()
            .as_mut()
            .unwrap()
            .produce_event_callback
            .as_mut()
        {
            callback(event);
        }
    });
}
