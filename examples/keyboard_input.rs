extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut window_manager = WindowManager::new().build().unwrap();
    let gl = window_manager.gl_context();

    let window = window_manager
        .new_window()
        .title("Keyboard Input Example")
        .build()
        .unwrap();
    let mut color = (0.0, 0.0, 0.0, 1.0);

    run(move |event| unsafe {
        match event {
            Event::ButtonDown {
                button,
                scancode: _,
            } => {
                match button {
                    Button::R => color = (1.0, 0.0, 0.0, 1.0),
                    Button::G => color = (0.0, 1.0, 0.0, 1.0),
                    Button::B => color = (0.0, 0.0, 1.0, 1.0),
                    _ => {}
                }
                println!("Button pressed: {:?}", button)
            }
            Event::Draw => {
                gl.clear_color(color.0, color.1, color.2, color.3);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                // When we're done rendering swap the window buffers to display to the screen.
                window_manager.swap_buffers(&window);
            }
            _ => {}
        }
    });
}
