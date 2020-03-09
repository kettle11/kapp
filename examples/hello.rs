extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut app = App::new().build().unwrap();
    let gl = app.gl_context();
    let window = app.new_window().build(&app).unwrap();

    unsafe {
        // gl.clear_color(0.0, 1.0, 1.0, 1.0);
        // gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
    }
    // Run forever
    let mut color = 0.0;
    let mut now = std::time::Instant::now();

    app.run(move |event, app| match event {
        Event::Draw => {
            unsafe {
                gl.clear_color(0.0, color, 1.0, 1.0);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            }
            color += 0.01;
            // now = std::time::Instant::now();

            // When we're done rendering swap the window buffers to display to the screen.
            app.swap_buffers();

            println!("Elapsed: {:?}", now.elapsed().as_millis());
            // app.request_frame();
            now = std::time::Instant::now();
        }
        _ => {}
    });
}
