extern crate gl;
extern crate windowing;
use windowing::*;

fn main() {
    // Create a new window manager with default settings.
    let mut window_manager = WindowManager::new();
    let window = window_manager.new_window("Mouse Move Example").unwrap();

    // There's not yet a way to query for the window size,
    // so falsely assume an initial value of 500.
    let mut window_width = 500.0;
    let mut window_height = 500.0;
    let mut color = (0.0, 0.0, 0.0, 1.0);

    run(move |event| unsafe {
        match event {
            Event::ResizedWindow { width, height } => {
                window_width = width as f32;
                window_height = height as f32;
            }
            Event::MouseMoved { x, y } => {
                println!("Mouse moved! X: {:?}, Y:{:?}", x, y);
                color = (x / window_width, 0.0, y / window_height, 1.0);
            }
            Event::Draw => {
                gl::ClearColor(color.0, color.1, color.2, color.3);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                window_manager.swap_buffers(&window);
            }
            _ => {}
        }
    });
}
