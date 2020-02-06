extern crate gl;
extern crate windowing;
use windowing::*;

fn draw_to_window(window_manager: &WindowManager, window: &Window, r: f32, g: f32, b: f32) {
    window_manager.make_current(window).unwrap();
    unsafe {
        gl::ClearColor(r, g, b, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
    // When we're done rendering swap the window buffers to display to the screen.
    window_manager.swap_buffers(window);
}

fn main() {
    // Create a new window manager with default settings.
    let mut window_manager = WindowManager::new();
    let window_red = window_manager.new_window("Window Red").unwrap();
    let window_blue = window_manager.new_window("Window Blue").unwrap();

    // Run forever
    run(move |event| match event {
        Event::Draw => {
            draw_to_window(&window_manager, &window_red, 1.0, 0.0, 0.0);
            draw_to_window(&window_manager, &window_blue, 0.0, 0.0, 1.0);
        }
        _ => {}
    });
}
