extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::Event::*;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut app = Application::new();
    let window = app.new_window().title("Mouse Down").build().unwrap();
    let mut gl_context = GLContext::new().build().unwrap(); // Create a gl_context for the app

    gl_context.set_window(Some(&window)).unwrap();
    let gl = gl_context.glow_context(); // Create a glow gl context for gl calls.

    // There's not yet a way to query for the window size,
    // so falsely assume an initial value of 500.
    let mut color = (0.0, 0.0, 0.0, 1.0);

    app.run(move |app, event| unsafe {
        match event {
            MouseButtonDown { button, .. } => match button {
                MouseButton::Left => {
                    println!("Left mouse button pressed!");
                    println!("Painting the window Red");

                    color = (1.0, 0.0, 0.0, 1.0);
                }
                MouseButton::Middle => {
                    println!("Middle mouse button pressed!");
                    println!("Painting the window Green");
                    color = (0.0, 1.0, 0.0, 1.0);
                }
                MouseButton::Right => {
                    println!("Right mouse button pressed!");
                    println!("Painting the window Blue");
                    color = (0.0, 0.0, 1.0, 1.0);
                }
                _ => {}
            },
            Draw => {
                gl_context.make_current();
                gl.clear_color(color.0, color.1, color.2, color.3);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                gl_context.swap_buffers();
                app.request_frame();
            }
            _ => {}
        }
    });
}
