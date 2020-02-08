extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut window_manager = WindowManager::new().build().unwrap();
    let gl = window_manager.gl_context();
    let window = window_manager
        .new_window()
        .title("Mouse Move")
        .dimensions(500, 500)
        .build()
        .unwrap();

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
                gl.clear_color(color.0, color.1, color.2, color.3);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                window_manager.swap_buffers(&window);
            }
            _ => {}
        }
    });
}
