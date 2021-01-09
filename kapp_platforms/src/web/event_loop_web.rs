use super::keys_web;
use kapp_platform_common::*;
use kwasm::*;
use std::ffi::c_void;
use std::time::Duration;

static mut CALLBACK: Option<Box<dyn FnMut(Event)>> = None;

static mut KAPP_MODULE: KWasmModule = KWasmModule::null();

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
            KAPP_MODULE,
            HostCommands::GetCanvasSize as kwasm::Command,
            float_data.as_mut_ptr(),
        );

        (float_data[0], float_data[1])
    }
}
*/

fn request_animation_frame_callback() {
    // Need to check for client resize here.
    // By comparing canvas width to its client width
    send_event(Event::Draw {
        window_id: WindowId::new(0 as *mut std::ffi::c_void),
    });
}

fn pointer_source_from_f64(f: f64) -> PointerSource {
    match f as u32 {
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

fn pointer_moved(x: f64, y: f64, pointer_enum: f64, time_stamp: f64) {
    send_event(Event::PointerMoved {
        x,
        y,
        source: pointer_source_from_f64(pointer_enum),
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
    kwasm::log(&format!("Mouse moved: {:?} {:?}", x, y));
}

fn pointer_down(x: f64, y: f64, pointer_enum: f64, button: f64, time_stamp: f64) {
    send_event(Event::PointerDown {
        button: button_from_f64(button), // This is incorrect
        x,
        y,
        source: pointer_source_from_f64(pointer_enum),
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
    kwasm::log(&format!("Pointer down: {:?} {:?}", x, y));
}

fn pointer_up(x: f64, y: f64, pointer_enum: f64, button: f64, time_stamp: f64) {
    send_event(Event::PointerUp {
        button: button_from_f64(button), // This is incorrect
        x,
        y,
        source: pointer_source_from_f64(pointer_enum),
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
    kwasm::log(&format!("Pointer up: {:?} {:?}", x, y));
}

fn key_down(time_stamp: f64) {
    let key = get_passed_str();
    send_event(Event::KeyDown {
        key: keys_web::virtual_keycode_to_key(key),
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    })
}

fn key_up(time_stamp: f64) {
    let key = get_passed_str();
    send_event(Event::KeyUp {
        key: keys_web::virtual_keycode_to_key(key),
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    })
}
fn key_repeat(time_stamp: f64) {
    let key = get_passed_str();
    send_event(Event::KeyRepeat {
        key: keys_web::virtual_keycode_to_key(key),
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    })
}

fn character_received(_time_stamp: f64) {
    let character = get_passed_str().chars().next().unwrap();
    send_event(Event::CharacterReceived { character })
}

fn scroll(delta_x: f64, delta_y: f64, time_stamp: f64) {
    send_event(Event::Scroll {
        delta_x,
        delta_y,
        window_id: WindowId::new(0 as *mut std::ffi::c_void),
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
}

// Note that 'feel' adjustments are made on the Javascript side to make this match
// Mac platform behavior. But that may be a bad idea.
fn pinch(delta: f64, time_stamp: f64) {
    send_event(Event::PinchGesture {
        delta,
        timestamp: Duration::from_secs_f64(time_stamp * 1000.0),
    });
}

#[repr(u32)]
enum HostCommands {
    RequestAnimationFrame = 0,
    //GetCanvasSize = 1,
    SetCallbacks = 2,
}

pub(crate) fn request_animation_frame() {
    unsafe {
        // Register the request animation frame callback
        kwasm::send_message_with_pointer_to_host(
            KAPP_MODULE,
            HostCommands::RequestAnimationFrame as kwasm::Command,
            request_animation_frame_callback as *mut c_void,
        );
    }
}

pub fn run<T>(callback: T)
where
    T: 'static + FnMut(Event),
{
    // Register the extra Javascript code we need
    unsafe {
        CALLBACK = Some(Box::new(Box::new(callback)));

        KAPP_MODULE = register_module(include_str!("kapp_module.js"));

        kwasm::send_message_with_data_to_host(
            KAPP_MODULE,
            HostCommands::SetCallbacks as kwasm::Command,
            &mut [
                pointer_moved as *mut c_void,      // 0
                pointer_down as *mut c_void,       // 1
                pointer_up as *mut c_void,         // 2
                key_down as *mut c_void,           // 3
                key_up as *mut c_void,             // 4
                scroll as *mut c_void,             // 5
                key_repeat as *mut c_void,         // 6,
                character_received as *mut c_void, // 7
                pinch as *mut c_void,              // 8
            ],
        );
    }
}
