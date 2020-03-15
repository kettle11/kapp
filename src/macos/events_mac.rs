use super::apple::*;
use super::application_mac::{
    get_window_data, ApplicationData, ApplicationInstanceData, WindowId, WindowState,
    INSTANCE_DATA_IVAR_ID,
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::{Event, Key, MouseButton};
// ------------------------ Window Events --------------------------

extern "C" fn window_did_move(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);
    unsafe {
        let backing_scale = window_data.backing_scale;
        let frame: CGRect = msg_send![window_data.ns_window, frame];
        self::produce_event(
            this,
            crate::Event::WindowMoved {
                x: (frame.origin.x * backing_scale) as u32,
                y: (frame.origin.y * backing_scale) as u32,
                window_id: WindowId {
                    ns_window: window_data.ns_window,
                },
            },
        );
    }
}
extern "C" fn window_did_miniaturize(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);
    window_data.window_state = WindowState::Minimized;
    self::produce_event(
        this,
        Event::WindowMinimized {
            window_id: WindowId {
                ns_window: window_data.ns_window,
            },
        },
    );
}

extern "C" fn window_did_deminiaturize(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);
    window_data.window_state = WindowState::Windowed; // Is this correct if the window immediately fullscreens?
    self::produce_event(
        this,
        Event::WindowRestored {
            window_id: WindowId {
                ns_window: window_data.ns_window,
            },
        },
    );
}

extern "C" fn window_did_enter_fullscreen(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);
    window_data.window_state = WindowState::Fullscreen;
    self::produce_event(
        this,
        Event::WindowFullscreened {
            window_id: WindowId {
                ns_window: window_data.ns_window,
            },
        },
    );
}
extern "C" fn window_did_exit_fullscreen(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);
    window_data.window_state = WindowState::Windowed; // Is this correct if the window immediately minimizes?
    self::produce_event(
        this,
        Event::WindowRestored {
            window_id: WindowId {
                ns_window: window_data.ns_window,
            },
        },
    );
}
extern "C" fn window_did_resize(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);

    unsafe {
        let backing_scale = window_data.backing_scale;
        let frame: CGRect = msg_send![window_data.ns_window, frame];
        self::produce_event(
            this,
            crate::Event::WindowResized {
                width: (frame.size.width * backing_scale) as u32,
                height: (frame.size.height * backing_scale) as u32,
                window_id: WindowId {
                    ns_window: window_data.ns_window,
                },
            },
        );
    }
}

extern "C" fn window_did_change_backing_properties(this: &Object, _sel: Sel, _event: *mut Object) {
    unsafe {
        let window_data = get_window_data(this);
        let old_scale = window_data.backing_scale;
        let new_scale: CGFloat = msg_send![this, backingScaleFactor];

        // The color space could have changed, not the backing scale.
        // So check here to make sure the scale has actually changed.
        // However the check doesn't matter as no event is sent (yet!)
        if old_scale != new_scale {
            window_data.backing_scale = new_scale;
        }
    }
}

extern "C" fn window_did_become_key(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);
    self::produce_event(
        this,
        crate::Event::WindowGainedFocus {
            window_id: WindowId {
                ns_window: window_data.ns_window,
            },
        },
    );
}

extern "C" fn window_did_resign_key(this: &Object, _sel: Sel, _event: *mut Object) {
    let window_data = get_window_data(this);
    self::produce_event(
        this,
        crate::Event::WindowLostFocus {
            window_id: WindowId {
                ns_window: window_data.ns_window,
            },
        },
    );
}

extern "C" fn window_should_close(this: &Object, _sel: Sel, _event: *mut Object) -> BOOL {
    let window_data = get_window_data(this);
    self::produce_event(
        this,
        crate::Event::WindowCloseRequested {
            window_id: WindowId {
                ns_window: window_data.ns_window,
            },
        },
    );
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
    NO
}

// https://developer.apple.com/documentation/appkit/nsapplicationdelegate/1428642-applicationshouldterminate?language=objc
extern "C" fn application_should_terminate(
    this: &Object,
    _sel: Sel,
    _event: *mut Object,
) -> NSUInteger {
    let application_data = get_application_data(this);
    submit_event(&(*application_data).application_data, Event::Quit);

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
extern "C" fn key_down(this: &Object, _sel: Sel, event: *mut Object) {
    unsafe {
        let key_code = msg_send![event, keyCode];
        let repeat: bool = msg_send![event, isARepeat];
        let key = super::keys_mac::virtual_keycode_to_key(key_code);
        let event = if repeat {
            crate::Event::KeyRepeat { key }
        } else {
            crate::Event::KeyDown { key }
        };
        self::produce_event(this, event);
    }
}

extern "C" fn key_up(this: &Object, _sel: Sel, event: *mut Object) {
    unsafe {
        let key_code = msg_send![event, keyCode];
        self::produce_event(
            this,
            crate::Event::KeyUp {
                key: super::keys_mac::virtual_keycode_to_key(key_code),
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
    const KEYS: [Key; 8] = [
        Key::LeftShift,
        Key::RightShift,
        Key::LeftControl,
        Key::RightControl,
        Key::LeftAlt,
        Key::RightAlt,
        Key::Meta,
        Key::Meta,
    ];

    let window_data = get_window_data(this);

    let modifier_flags_old = window_data.application_data.borrow().modifier_flags;

    let modifier_flags_new: NSUInteger = unsafe { msg_send![event, modifierFlags] };

    let flag_state_old = get_modifier_state(modifier_flags_old);
    let flag_state_new = get_modifier_state(modifier_flags_new);

    for i in 0..8 {
        if !flag_state_old[i] && flag_state_new[i] {
            produce_event(this, crate::Event::KeyDown { key: KEYS[i] })
        }

        if flag_state_old[i] && !flag_state_new[i] {
            produce_event(this, crate::Event::KeyUp { key: KEYS[i] })
        }
    }

    (*window_data).application_data.borrow_mut().modifier_flags = modifier_flags_new;
}

extern "C" fn mouse_moved(this: &Object, _sel: Sel, event: *mut Object) {
    unsafe {
        let window_data = get_window_data(this);
        let backing_scale = (*window_data).backing_scale;

        let window_point: NSPoint = msg_send![event, locationInWindow];
        let x = window_point.x * backing_scale;
        let y = window_point.y * backing_scale; // Don't flip because 0 is bottom left on MacOS

        self::produce_event(
            this,
            crate::Event::MouseMoved {
                x: x as f32,
                y: y as f32,
            },
        );
    }
}

extern "C" fn mouse_down(this: &Object, _sel: Sel, _event: *mut Object) {
    self::produce_event(
        this,
        crate::Event::MouseButtonDown {
            button: MouseButton::Left,
        },
    );
}

extern "C" fn mouse_up(this: &Object, _sel: Sel, _event: *mut Object) {
    self::produce_event(
        this,
        crate::Event::MouseButtonUp {
            button: MouseButton::Left,
        },
    );
}

extern "C" fn right_mouse_down(this: &Object, _sel: Sel, _event: *mut Object) {
    self::produce_event(
        this,
        crate::Event::MouseButtonDown {
            button: MouseButton::Right,
        },
    );
}

extern "C" fn right_mouse_up(this: &Object, _sel: Sel, _event: *mut Object) {
    self::produce_event(
        this,
        crate::Event::MouseButtonUp {
            button: MouseButton::Right,
        },
    );
}

extern "C" fn other_mouse_down(this: &Object, _sel: Sel, event: *mut Object) {
    let number: NSInteger = unsafe { msg_send![event, buttonNumber] };
    let button = match number {
        // Are these correct?
        4 => MouseButton::Middle,
        8 => MouseButton::Extra1,
        16 => MouseButton::Extra2,
        _ => MouseButton::Unknown,
    };
    self::produce_event(this, crate::Event::MouseButtonDown { button });
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
    self::produce_event(this, crate::Event::MouseButtonUp { button });
}

pub fn add_view_events_to_decl(decl: &mut ClassDecl) {
    unsafe {
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

// 'this' can be either a Window delegate or a view delegate.
// Despite being different types both have iVars attached for inner_window_data
fn produce_event(this: &Object, event: crate::Event) {
    let window_data = get_window_data(this);
    submit_event(&(*window_data).application_data, event);
}

fn get_application_data(this: &Object) -> &mut ApplicationInstanceData {
    unsafe {
        let data: *mut std::ffi::c_void = *this.get_ivar(INSTANCE_DATA_IVAR_ID);
        &mut *(data as *mut ApplicationInstanceData)
    }
}

pub fn submit_event(application_data: &Rc<RefCell<ApplicationData>>, event: Event) {
    let mut program_callback = application_data.borrow_mut().program_callback.take();

    if let Some(callback) = program_callback.as_mut() {
        callback(event);

        // Process any events that may have been queued during the above callback.
        // Care is taken to not borrow the application_data during the callback.
        let mut queued_event = application_data.borrow_mut().event_queue.pop();

        while let Some(event) = queued_event {
            callback(event);
            queued_event = application_data.borrow_mut().event_queue.pop();
        }
    } else {
        // If this event is created during a program callback then it can't be processed immediately
        // and must be enqueued.
        application_data.borrow_mut().event_queue.push(event);
    }

    application_data.borrow_mut().program_callback = program_callback;
}
