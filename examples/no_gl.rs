extern crate kettlewin;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut window_manager = WindowManager::new().build().unwrap();
    let window = window_manager.new_window().build().unwrap();

    // Run forever
    run(move |event| match event {
        Event::Draw => {
            // When we're done rendering swap the window buffers to display to the screen.
            window_manager.swap_buffers(&window);
        }
        _ => {}
    });
}
