use kettlewin::*;
use ogl33::*;

fn main() {
    // Create a new window manager with default settings.
    let mut window_manager = WindowManager::new().build().unwrap();
    unsafe { load_gl_with(window_manager.gl_loader_c_string()); }
    let window = window_manager.new_window().build().unwrap();

    // Run forever
    run(move |event| match event {
        Event::Draw => {
            unsafe {
                glClearColor(0.0, 1.0, 1.0, 1.0);
                glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            }
            // When we're done rendering swap the window buffers to display to the screen.
            window_manager.swap_buffers(&window);
        }
        _ => {}
    });
}
