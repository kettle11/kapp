extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

fn main() {
    // Create a new application with default settings.
    let mut app = Application::new().build().unwrap();
    let window = app.new_window().title("Hello").build().unwrap();
    let mut gl_context = GLContext::new().build().unwrap(); // Create a gl_context for the app

    gl_context.set_window(Some(&window)).unwrap();
    let gl = gl_context.glow_context(); // Create a glow gl context for gl calls.

    // Run forever
    let mut color = (1.0, 0.0, 0.0, 1.0);

    app.event_loop().run(move |event| match event {
        Event::WindowCloseRequested { .. } => app.quit(),
        Event::WindowResized { .. } => gl_context.update_target(), // This call updates the window backbuffer to match the new window size.
        Event::TrackpadTouch { x, y } => {
            if x < 0.333 {
                color = (y, color.1, color.2, 1.);
            } else if x < 0.66 {
                color = (color.0, y, color.2, 1.);
            } else {
                color = (color.0, color.1, y, 1.);
            }
        }
        Event::KeyDown { key } => match key {
            Key::F => {
                window.fullscreen();
            }
            Key::Q => {
                app.quit();
            }
            _ => {}
        },
        Event::Draw => {
            app.set_mouse_position(0, 500);
            unsafe {
                gl.clear_color(color.0, color.1, color.2, 1.0);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            }

            gl_context.swap_buffers(); // Swaps the currently bound window. Blocks if vSync is used
            app.request_frame();
        }
        _ => {}
    });
}
