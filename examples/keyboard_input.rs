extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut app = App::new().build().unwrap();
    let gl = app.gl_context();

    let _window = app
        .new_window()
        .title("Keyboard Input Example")
        .build(&app)
        .unwrap();
    let mut color = (0.0, 0.0, 0.0, 1.0);

    app.run(move |event, app| unsafe {
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
                app.swap_buffers();
            }
            _ => {}
        }
    });
}
