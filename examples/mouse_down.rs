extern crate gl;
extern crate kettlewin;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut window_manager = WindowManager::new().unwrap();
    gl::load_with(window_manager.gl_loader());

    let window = window_manager
        .new_window("Mouse Down Example", Some(600), Some(600))
        .unwrap();

    // There's not yet a way to query for the window size,
    // so falsely assume an initial value of 500.
    let mut color = (0.0, 0.0, 0.0, 1.0);

    run(move |event| unsafe {
        match event {
            Event::MouseDown { button } => match button {
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
            },
            Event::Draw => {
                gl::ClearColor(color.0, color.1, color.2, color.3);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                window_manager.swap_buffers(&window);
            }
            _ => {}
        }
    });
}
