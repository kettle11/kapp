use super::keys_web;
use kapp_platform_common::*;

use std::time::Duration;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

static mut CALLBACK: Option<Box<dyn FnMut(Event)>> = None;
static mut REQUEST_ANIMATION_FRAME_CLOSURE: Option<Closure<dyn FnMut()>> = None;
static mut REQUEST_FULLSCREEN_CLOSURE: Option<Closure<dyn FnMut()>> = None;
static mut CANVAS_HEIGHT: u32 = 0;

fn send_event(event: Event) {
    unsafe {
        (CALLBACK.as_mut().unwrap())(event);
    }
}

pub fn run<T>(callback: T)
where
    T: 'static + FnMut(Event),
{
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    // While the following is 'unsafe' and uses global data in a funky way, it's actually safe because web's main loop is single threaded.
    // An alternative approach is documented here: https://rustwasm.github.io/docs/wasm-bindgen/examples/request-animation-frame.html
    // It may be better, but for now I found the following simpler to understand and implement.
    unsafe {
        CALLBACK = Some(Box::new(Box::new(callback)));
        {
            let canvas = canvas.clone();
            REQUEST_ANIMATION_FRAME_CLOSURE = Some(Closure::wrap(Box::new(move || {
                let canvas_client_width = canvas.client_width() as u32;
                let canvas_client_height = canvas.client_height() as u32;
                if canvas_client_width != canvas.width() || canvas_client_height != canvas.height()
                {
                    // This may impact smoothness of resizing.
                    canvas.set_width(canvas_client_width);
                    canvas.set_height(canvas_client_height);

                    CANVAS_HEIGHT = canvas_client_height;

                    send_event(Event::WindowResized {
                        width: canvas_client_width,
                        height: canvas_client_height,
                        window_id: WindowId::new(0 as *mut std::ffi::c_void),
                    });
                }

                send_event(Event::Draw {
                    window_id: WindowId::new(0 as *mut std::ffi::c_void),
                });
                // request_animation_frame(REQUEST_ANIMATION_FRAME_CLOSURE.as_ref().unwrap())
            })
                as Box<dyn FnMut()>));

            REQUEST_FULLSCREEN_CLOSURE =
                Some(Closure::wrap(
                    Box::new(move || println!("Fullscreened?")) as Box<dyn FnMut()>
                ));
        }

        // Pointer move event
        let pointer_move = Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
            let (x, y) = get_pointer_position(&event);
            send_event(Event::PointerMoved {
                x,
                y,
                source: get_pointer_type(&event),
                timestamp: Duration::from_secs_f64(event.time_stamp() * 1000.0),
            });
        }) as Box<dyn FnMut(web_sys::PointerEvent)>);
        canvas.set_onpointermove(Some(pointer_move.as_ref().unchecked_ref()));
        pointer_move.forget();

        // Pointer down event
        let pointer_down = Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
            let (x, y) = get_pointer_position(&event);

            send_event(Event::PointerDown {
                x,
                y,
                source: get_pointer_type(&event),
                button: match event.button() {
                    0 => PointerButton::Primary,
                    1 => PointerButton::Auxillary,
                    2 => PointerButton::Secondary,
                    3 => PointerButton::Extra1,
                    4 => PointerButton::Extra2,
                    _ => PointerButton::Unknown,
                },
                timestamp: Duration::from_secs_f64(event.time_stamp() * 1000.0),
            });
        }) as Box<dyn FnMut(web_sys::PointerEvent)>);
        canvas.set_onpointerdown(Some(pointer_down.as_ref().unchecked_ref()));
        pointer_down.forget();

        // Pointer up event
        let pointer_up = Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
            let (x, y) = get_pointer_position(&event);

            send_event(Event::PointerUp {
                x,
                y,
                source: get_pointer_type(&event),
                button: match event.button() {
                    0 => PointerButton::Primary,
                    1 => PointerButton::Auxillary,
                    2 => PointerButton::Secondary,
                    3 => PointerButton::Extra1,
                    4 => PointerButton::Extra2,
                    _ => PointerButton::Unknown,
                },
                timestamp: Duration::from_secs_f64(event.time_stamp() * 1000.0),
            });
        }) as Box<dyn FnMut(web_sys::PointerEvent)>);
        canvas.set_onpointerup(Some(pointer_up.as_ref().unchecked_ref()));
        pointer_up.forget();

        // Key down event
        let keydown = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let key_event = if event.repeat() {
                Event::KeyRepeat {
                    key: keys_web::virtual_keycode_to_key(&event.code()),
                    timestamp: Duration::from_secs_f64(event.time_stamp() * 1000.0),
                }
            } else {
                Event::KeyDown {
                    key: keys_web::virtual_keycode_to_key(&event.code()),
                    timestamp: Duration::from_secs_f64(event.time_stamp() * 1000.0),
                }
            };

            send_event(key_event);
            event
                .dyn_into::<web_sys::Event>()
                .unwrap()
                .prevent_default();
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        document.set_onkeydown(Some(keydown.as_ref().unchecked_ref()));
        keydown.forget();

        // Key up event
        let keyup = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            send_event(Event::KeyUp {
                key: keys_web::virtual_keycode_to_key(&event.code()),
                timestamp: Duration::from_secs_f64(event.time_stamp() * 1000.0),
            });
            event
                .dyn_into::<web_sys::Event>()
                .unwrap()
                .prevent_default();
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        document.set_onkeyup(Some(keyup.as_ref().unchecked_ref()));
        keyup.forget();

        // Scroll event (from browser 'wheel' event)
        // Note that the actual browser 'scroll' events aren't used here as they can
        // occur with the scrollbar.
        // Values are reversed because they were the opposite of native MacOS.
        // However it'd be better to know what the cross platform expectation is.
        let wheel = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
            if event.ctrl_key() {
                // This is a bit weird, but if a pinch gesture is performed
                // the ctrl modifier is set.
                // This is the simplest way to disambiguate it.
                send_event(Event::PinchGesture {
                    // 0.02 is a completely arbitrary number to make this value more similar
                    // to what native MacOS produces.
                    // Is this a good idea at all?
                    // Should this library even make such adjustments?
                    // Is there a way to find an actual scale factor instead of a guess?
                    delta: -event.delta_y() * 0.02,
                    timestamp: Duration::from_secs_f64(event.time_stamp() * 1000.0),
                });
            } else {
                send_event(Event::Scroll {
                    delta_x: -event.delta_x(),
                    delta_y: -event.delta_y(),
                    window_id: WindowId::new(0 as *mut std::ffi::c_void),
                    timestamp: Duration::from_secs_f64(event.time_stamp() * 1000.0),
                });
            }
            event
                .dyn_into::<web_sys::Event>()
                .unwrap()
                .prevent_default();
        }) as Box<dyn FnMut(web_sys::WheelEvent)>);
        canvas.set_onwheel(Some(wheel.as_ref().unchecked_ref()));
        wheel.forget();
        // Finally, start the draw loop.
        request_frame();
    }
}

fn get_pointer_type(event: &web_sys::PointerEvent) -> PointerSource {
    match event.pointer_type().as_str() {
        "mouse" => PointerSource::Mouse,
        "pen" => PointerSource::Pen,
        "touch" => PointerSource::Touch,
        _ => PointerSource::Unknown,
    }
}

fn get_pointer_position(event: &web_sys::PointerEvent) -> (f64, f64) {
    // 0,0 is the upper left of the canvas on web, so no transformations need to be performed.
    (event.client_x().into(), event.client_y().into())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn request_frame() {
    unsafe {
        request_animation_frame(REQUEST_ANIMATION_FRAME_CLOSURE.as_ref().unwrap());
    }
}

pub fn request_fullscreen() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    canvas.request_fullscreen().unwrap();
}
