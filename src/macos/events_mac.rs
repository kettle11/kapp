use super::apple::*;

// ------------------------ Window Events --------------------------
extern "C" fn window_moved(_this: &Object, _sel: Sel, _event: *mut Object) {}
extern "C" fn window_did_resize(_this: &Object, _sel: Sel, _event: *mut Object) {
    // TEST_VIEW needs to be replaced with the actual window view.
    /*
    unsafe {
        if let Some(data) = TEST_VIEW.as_ref() {
            let rect: NSRect = msg_send![data.view, frame];
            let window = get_window_data(this);
            let new_name = NSString::new("resized");
            let () = msg_send![window, setTitle: new_name.raw];

            let screen: *mut Object = msg_send![window, screen];

            let scale_factor: CGFloat = msg_send![screen, backingScaleFactor];

            println!("Backing scale factor: {:?}", scale_factor);

            println!("RECT: {:?}", rect);
            let width = rect.size.width;
            let height = rect.size.height;

            println!(
                "RESIZED SCALED: {:?}, {:?}",
                width * scale_factor,
                height * scale_factor
            );

            if let Some(app) = APP.as_mut() {
                // The following line is needed to set the view current before updating it.
                // let () = msg_send![app.gl_context, setView: ];
                // let () = msg_send![app.gl_context, update];
            }

            self::produce_event(crate::Event::ResizedWindow {
                width: width as u32,
                height: height as u32,
            });
        }
    }*/
}
// ------------------------ End Window Events --------------------------

// ------------------------ View Events --------------------------
extern "C" fn key_down(_this: &Object, _sel: Sel, event: *mut Object) {
    unsafe {
        let key_code = msg_send![event, keyCode];
        self::produce_event(crate::Event::ButtonDown {
            button: super::keys_mac::virtual_keycode_to_key(key_code),
            scancode: 0,
        });
    }
}

extern "C" fn key_up(_this: &Object, _sel: Sel, event: *mut Object) {
    unsafe {
        let key_code = msg_send![event, keyCode];
        self::produce_event(crate::Event::ButtonUp {
            button: super::keys_mac::virtual_keycode_to_key(key_code),
            scancode: 0,
        });
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

        self::produce_event(crate::Event::MouseMoved {
            x: x as f32,
            y: y as f32,
        });
    }
}
// ------------------------ End View Events --------------------------

pub fn produce_event(event: crate::Event) {
    unsafe {
        if let Some(program_callback) = super::window_manager_macos::PROGRAM_CALLBACK.as_mut() {
            program_callback(event);
        }
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
    }
}
