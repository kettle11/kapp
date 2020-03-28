use super::keys_web;
use crate::{Event, MouseButton, WindowId};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

static mut CALLBACK: Option<Box<dyn FnMut(Event)>> = None;
static mut REQUEST_ANIMATION_FRAME_CLOSURE: Option<Closure<dyn FnMut()>> = None;
static mut REQUEST_FULLSCREEN_CLOSURE: Option<Closure<dyn FnMut()>> = None;

fn send_event(event: Event) {
    unsafe {
        (CALLBACK.as_mut().unwrap())(event);
    }
}

pub fn run<T>(callback: T)
where
    T: 'static + FnMut(crate::Event),
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
                    canvas.set_width(canvas_client_width);
                    canvas.set_height(canvas_client_height);
                    send_event(Event::WindowResized {
                        width: canvas_client_width,
                        height: canvas_client_height,
                        window_id: WindowId::new(0 as *mut std::ffi::c_void),
                    });
                }

                send_event(Event::Draw);
                // request_animation_frame(REQUEST_ANIMATION_FRAME_CLOSURE.as_ref().unwrap())
            })
                as Box<dyn FnMut()>));

            REQUEST_FULLSCREEN_CLOSURE =
                Some(Closure::wrap(
                    Box::new(move || println!("Fullscreened?")) as Box<dyn FnMut()>
                ));
        }

        // Mouse move event
        let mouse_move = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            send_event(Event::MouseMoved {
                x: event.client_x() as f32,
                y: event.client_y() as f32,
            });
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        canvas.set_onmousemove(Some(mouse_move.as_ref().unchecked_ref()));
        mouse_move.forget();

        // Mouse down event
        let mouse_down = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            send_event(Event::MouseButtonDown {
                button: match event.button() {
                    0 => MouseButton::Left,
                    1 => MouseButton::Middle,
                    2 => MouseButton::Right,
                    3 => MouseButton::Extra1,
                    4 => MouseButton::Extra2,
                    _ => MouseButton::Unknown,
                },
            });
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        canvas.set_onmousedown(Some(mouse_down.as_ref().unchecked_ref()));
        mouse_down.forget();

        // Mouse up event
        let mouse_up = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            send_event(Event::MouseButtonUp {
                button: match event.button() {
                    0 => MouseButton::Left,
                    1 => MouseButton::Middle,
                    2 => MouseButton::Right,
                    3 => MouseButton::Extra1,
                    4 => MouseButton::Extra2,
                    _ => MouseButton::Unknown,
                },
            });
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        canvas.set_onmouseup(Some(mouse_up.as_ref().unchecked_ref()));
        mouse_up.forget();

        // Key down event
        let keydown = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let key_event = if event.repeat() {
                Event::KeyRepeat {
                    key: keys_web::virtual_keycode_to_key(&event.code()),
                }
            } else {
                Event::KeyDown {
                    key: keys_web::virtual_keycode_to_key(&event.code()),
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
            });
            event
                .dyn_into::<web_sys::Event>()
                .unwrap()
                .prevent_default();
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        document.set_onkeyup(Some(keyup.as_ref().unchecked_ref()));
        keyup.forget();
        // Finally, start the draw loop.
        request_frame();
    }
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
