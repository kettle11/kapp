extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut app = Application::new().build().unwrap();
    let mut gl_context = GLContext::new().build().unwrap(); // Create a gl_context for the app
    let gl = gl_context.glow_context(); // Create a gl context (for gl calls) using the glow crate.

    let window = app
        .new_window()
        .title("Keyboard Input Example")
        .build()
        .unwrap();
    gl_context.set_window(Some(&window)).unwrap();

    let mut color = (0.0, 0.0, 0.0, 1.0);

    app.event_loop().run(move |event| unsafe {
        match event {
            Event::ButtonDown { button } => {
                match button {
                    Button::R => color = (1.0, 0.0, 0.0, 1.0),
                    Button::G => color = (0.0, 1.0, 0.0, 1.0),
                    Button::B => color = (0.0, 0.0, 1.0, 1.0),
                    _ => {}
                }
                println!("Button pressed: {:?}", button)
            }
            Event::ButtonUp { button } => println!("Button released: {:?}", button),
            Event::ButtonRepeat { button } => println!("Button repeated: {:?}", button),
            Event::Draw => {
                gl.clear_color(color.0, color.1, color.2, color.3);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                // When we're done rendering swap the window buffers to display to the screen.
                gl_context.swap_buffers();
            }
            _ => {}
        }
    });
}
