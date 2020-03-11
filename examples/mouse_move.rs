extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut app = Application::new().build().unwrap();
    let window = app
        .new_window()
        .title("Mouse Move")
        .dimensions(500, 500)
        .build()
        .unwrap();

    let gl_context = GLContext::new(); // Create a gl_context for the app
    gl_context.set_window(&window).unwrap();
    let gl = gl_context.glow_context(); // Create a glow gl context for gl calls.

    let mut window_width = 500.0;
    let mut window_height = 500.0;
    let mut color = (0.0, 0.0, 0.0, 1.0);

    app.event_loop().run(move |event| unsafe {
        match event {
            Event::ResizedWindow { width, height } => {
                window_width = width as f32;
                window_height = height as f32;
            }
            Event::MouseMoved { x, y } => {
                println!("Mouse moved! X: {:?}, Y:{:?}", x, y);
                color = (x / window_width, 0.0, y / window_height, 1.0);
            }
            Event::Draw => {
                gl.clear_color(color.0, color.1, color.2, color.3);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                gl_context.swap_buffers();
            }
            _ => {}
        }
    });
}
