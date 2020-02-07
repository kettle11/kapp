extern crate gl;
extern crate kettlewin;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut window_manager = WindowManager::new().unwrap();
    gl::load_with(window_manager.gl_loader());

    let window = window_manager
        .new_window("Keyboard Input Example", Some(600), Some(600))
        .unwrap();
    let mut color = (0.0, 0.0, 0.0, 1.0);

    run(move |event| unsafe {
        match event {
            Event::KeyDown { key, scancode: _ } => match key {
                Key::R => color = (1.0, 0.0, 0.0, 1.0),
                Key::G => color = (0.0, 1.0, 0.0, 1.0),
                Key::B => color = (0.0, 0.0, 1.0, 1.0),
                _ => {}
            },
            Event::Draw => {
                gl::ClearColor(color.0, color.1, color.2, color.3);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                // When we're done rendering swap the window buffers to display to the screen.
                window_manager.swap_buffers(&window);
            }
            _ => {}
        }
    });
}
