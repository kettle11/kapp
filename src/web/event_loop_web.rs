use crate::events::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

static mut CALLBACK: Option<Box<dyn FnMut(Event)>> = None;
static mut REQUEST_ANIMATION_FRAME_CLOSURE: Option<Closure<dyn FnMut()>> = None;

pub fn run<T>(callback: T)
where
    T: 'static + FnMut(Event),
{
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    // While the following is unsafe and uses global data in a funky way, it's actually safe because web's main loop is single threaded.
    // An alternative approach is documented here: https://rustwasm.github.io/docs/wasm-bindgen/examples/request-animation-frame.html
    // It may be better, but for now I found the following simpler to follow and implement.
    unsafe {
        CALLBACK = Some(Box::new(Box::new(callback)));

        REQUEST_ANIMATION_FRAME_CLOSURE = Some(Closure::wrap(Box::new(move || {
            (CALLBACK.as_mut().unwrap())(Event::Draw);
            request_animation_frame(REQUEST_ANIMATION_FRAME_CLOSURE.as_ref().unwrap())
        }) as Box<dyn FnMut()>));

        let mouse_move = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            (CALLBACK.as_mut().unwrap())(Event::MouseMoved {
                x: event.client_x() as f32,
                y: event.client_y() as f32,
            });
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        canvas.set_onmousemove(Some(mouse_move.as_ref().unchecked_ref()));
        mouse_move.forget();

        let mouse_down = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            (CALLBACK.as_mut().unwrap())(Event::MouseDown {
                button: match event.button() {
                    0 => MouseButton::Left,
                    1 => MouseButton::Middle,
                    2 => MouseButton::Right,
                    _ => MouseButton::Unknown,
                },
            });
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        canvas.set_onmousedown(Some(mouse_down.as_ref().unchecked_ref()));
        mouse_down.forget();

        request_animation_frame(REQUEST_ANIMATION_FRAME_CLOSURE.as_ref().unwrap());
    }
}
