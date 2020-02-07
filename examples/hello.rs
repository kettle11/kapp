extern crate gl;
extern crate kettlewin;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut window_manager = WindowManager::new().unwrap();
    gl::load_with(window_manager.gl_loader());
    let window = window_manager
        .new_window("Hello Example", Some(600), Some(600))
        .unwrap();

    // Run forever
    run(move |event| match event {
        Event::Draw => {
            unsafe {
                gl::ClearColor(0.0, 1.0, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }
            // When we're done rendering swap the window buffers to display to the screen.
            window_manager.swap_buffers(&window);
        }
        _ => {}
    });
}
