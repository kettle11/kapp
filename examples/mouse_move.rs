extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

fn main() {
    let mut window_width = 500;
    let mut window_height = 500;

    // Create a new window manager with default settings.
    let mut app = Application::new();
    let window = app
        .new_window()
        .title("Mouse Move")
        .dimensions(window_width, window_height)
        .build()
        .unwrap();

    let mut gl_context = GLContext::new().build().unwrap(); // Create a gl_context for the app
    gl_context.set_window(Some(&window)).unwrap();
    let gl = gl_context.glow_context(); // Create a glow gl context for gl calls.

    let mut color = (0.0, 0.0, 0.0, 1.0);

    let mut requested_frame = false;
    app.run(move |app, event| unsafe {
        match event {
            Event::WindowCloseRequested { .. } => app.quit(),
            Event::WindowResized { width, height, .. } => {
                window_width = width;
                window_height = height;
            }
            Event::MouseMoved { x, y, .. } => {
                println!("Mouse moved! X: {:?}, Y:{:?}", x, y);
                color = (x / window_width as f32, 0.0, y / window_height as f32, 1.0);

                println!("Event: {:?}", event);

                // By requesting a frame here the program only redraws when the mouse moves.
                // Multiple calls to request_frame will produce multiple draw events.
                // Multiple mouse events can occur per frame,
                // so check that we haven't already requested a frame.
                if !requested_frame {
                    requested_frame = true;
                    app.request_frame();
                }
            }
            Event::Draw => {
                gl_context.make_current();
                requested_frame = false;
                gl.clear_color(color.0, color.1, color.2, color.3);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                gl_context.swap_buffers();
            }
            _ => {}
        }
    });
}
