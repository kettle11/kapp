use super::keys_web;
use kapp_platform_common::*;
use kwasm::*;
use std::time::Duration;

thread_local! {
    pub static KAPP_LIBRARY: KWasmLibrary = KWasmLibrary::new(include_str!("kapp_library.js"));
}

static mut CALLBACK: Option<Box<dyn FnMut(Event)>> = None;

fn send_event(event: Event) {
    unsafe {
        (CALLBACK.as_mut().unwrap())(event);
    }
}

/*
fn request_canvas_size() -> (f32, f32) {
    unsafe {
        let mut float_data: [f32; 2] = [0., 0.];
        kwasm::send_message_with_pointer_to_host(
            KAPP_LIBRARY,
            HostCommands::GetCanvasSize as kwasm::Command,
            float_data.as_mut_ptr(),
        );

        (float_data[0], float_data[1])
    }
}
*/

fn pointer_source_from_u32(f: u32) -> PointerSource {
    match f {
        1 => PointerSource::Mouse,
        2 => PointerSource::Pen,
        3 => PointerSource::Touch,
        _ => PointerSource::Unknown,
    }
}

fn button_from_f64(f: f64) -> PointerButton {
    match f as u32 {
        0 => PointerButton::Primary,
        1 => PointerButton::Auxillary,
        2 => PointerButton::Secondary,
        3 => PointerButton::Extra1,
        4 => PointerButton::Extra2,
        _ => PointerButton::Unknown,
    }
}

#[no_mangle]
pub extern "C" fn kapp_on_window_resized(width: u32, height: u32) {
    send_event(Event::WindowResized {
        width,
        height,
        window_id: WindowId::new(0 as *mut std::ffi::c_void),
    });
}

#[no_mangle]
pub extern "C" fn kapp_on_animation_frame() {
    // Need to check for client resize here.
    // By comparing canvas width to its client width
    send_event(Event::Draw {
        window_id: WindowId::new(0 as *mut std::ffi::c_void),
    });
}

#[no_mangle]
pub extern "C" fn kapp_on_pointer_move(x: f64, y: f64, pointer_enum: u32, time_stamp: f64) {
    send_event(Event::PointerMoved {
        x,
        y,
        source: pointer_source_from_u32(pointer_enum),
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
}

#[no_mangle]
pub extern "C" fn kapp_on_mouse_move(delta_x: f64, delta_y: f64, time_stamp: f64) {
    send_event(Event::MouseMotion {
        delta_x,
        delta_y,
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
}

#[no_mangle]
pub extern "C" fn kapp_on_pointer_down(
    x: f64,
    y: f64,
    pointer_enum: u32,
    button: f64,
    time_stamp: f64,
) {
    send_event(Event::PointerDown {
        button: button_from_f64(button), // This is incorrect
        x,
        y,
        source: pointer_source_from_u32(pointer_enum),
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
}

#[no_mangle]
pub extern "C" fn kapp_on_pointer_up(
    x: f64,
    y: f64,
    pointer_enum: u32,
    button: f64,
    time_stamp: f64,
) {
    send_event(Event::PointerUp {
        button: button_from_f64(button), // This is incorrect
        x,
        y,
        source: pointer_source_from_u32(pointer_enum),
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
}

#[no_mangle]
pub extern "C" fn kapp_on_key_down(time_stamp: f64) {
    kwasm::DATA_FROM_HOST.with(|d| {
        let d = d.borrow();
        let key = std::str::from_utf8(&d).unwrap();
        send_event(Event::KeyDown {
            key: keys_web::virtual_keycode_to_key(key),
            timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
        })
    })
}

#[no_mangle]
pub extern "C" fn kapp_on_key_up(time_stamp: f64) {
    kwasm::DATA_FROM_HOST.with(|d| {
        let d = d.borrow();
        let key = std::str::from_utf8(&d).unwrap();
        send_event(Event::KeyUp {
            key: keys_web::virtual_keycode_to_key(key),
            timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
        })
    });
}
#[no_mangle]
pub extern "C" fn kapp_on_key_repeat(time_stamp: f64) {
    kwasm::DATA_FROM_HOST.with(|d| {
        let d = d.borrow();
        let key = std::str::from_utf8(&d).unwrap();
        send_event(Event::KeyRepeat {
            key: keys_web::virtual_keycode_to_key(key),
            timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
        });
    });
}

#[no_mangle]
pub extern "C" fn kapp_character_received(_time_stamp: f64) {
    kwasm::DATA_FROM_HOST.with(|d| {
        let d = d.borrow();
        let data = std::str::from_utf8(&d).unwrap();
        let character = data.chars().next().unwrap();
        send_event(Event::CharacterReceived { character })
    });
}

#[no_mangle]
pub extern "C" fn kapp_on_scroll(delta_x: f64, delta_y: f64, time_stamp: f64) {
    send_event(Event::Scroll {
        delta_x,
        delta_y,
        window_id: WindowId::new(0 as *mut std::ffi::c_void),
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
}

// Note that 'feel' adjustments are made on the Javascript side to make this match
// Mac platform behavior. But that may be a bad idea.
#[no_mangle]
pub extern "C" fn kapp_on_pinch(delta: f64, time_stamp: f64) {
    send_event(Event::PinchGesture {
        delta,
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
}

#[repr(u32)]
pub(crate) enum HostCommands {
    RequestAnimationFrame = 0,
    //GetCanvasSize = 1,
    SetCallbacks = 2,
    GetDevicePixelRatio = 3,
    GetWindowSize = 4,
    LockCursor = 5,
    UnlockCursor = 6,
}

pub(crate) fn request_animation_frame() {
    // Register the request animation frame callback
    KAPP_LIBRARY.with(|l| l.message(HostCommands::RequestAnimationFrame as kwasm::Command));
}

pub fn run<T>(callback: T)
where
    T: 'static + FnMut(Event),
{
    // Register the extra Javascript code we need
    unsafe {
        CALLBACK = Some(Box::new(Box::new(callback)));
    }
    KAPP_LIBRARY.with(|l| l.message(HostCommands::SetCallbacks as kwasm::Command));
}
