extern crate gl;
extern crate kettlewin;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut window_manager = WindowManager::new().build().unwrap();
    gl::load_with(window_manager.gl_loader());

    let window_red = window_manager
        .new_window()
        .title("Window Red")
        .build()
        .unwrap();
    let window_blue = window_manager
        .new_window()
        .title("Window Blue")
        .build()
        .unwrap();

    // Run forever
    run(move |event| match event {
        Event::Draw => {
            draw_to_window(&window_manager, &window_red, 1.0, 0.0, 0.0);
            draw_to_window(&window_manager, &window_blue, 0.0, 0.0, 1.0);
        }
        _ => {}
    });
}

fn draw_to_window(window_manager: &WindowManager, window: &Window, r: f32, g: f32, b: f32) {
    // If make_current fails the window may no longer be open.
    if window_manager.make_current(window).is_ok() {
        unsafe {
            gl::ClearColor(r, g, b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        // When we're done rendering swap the window buffers to display to the screen.
        window_manager.swap_buffers(window);
    }
}
