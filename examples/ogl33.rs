extern crate kettlewin;
extern crate ogl33;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut window_manager = WindowManager::new().build().unwrap();
    let gl = window_manager.gl_context();
    let window = window_manager.new_window().build().unwrap();

    // Run forever
    run(move |event| match event {
        Event::Draw => {
            unsafe {
                glClearColor(0.0, 1.0, 1.0, 1.0);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            }
            // When we're done rendering swap the window buffers to display to the screen.
            window_manager.swap_buffers(&window);
        }
        _ => {}
    });
}
