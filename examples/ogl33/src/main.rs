use kettlewin::*;
use ogl33::*;

fn main() {
    // Create a new window manager with default settings.
    let mut app = App::new().build().unwrap();
    unsafe {
        load_gl_with(app.gl_loader_c_string());
    }
    let window = app.new_window().build(&app).unwrap();

    // Run forever
    app.run(move |event, app| match event {
        Event::Draw => {
            unsafe {
                glClearColor(0.0, 1.0, 1.0, 1.0);
                glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            }
            // When we're done rendering swap the window buffers to display to the screen.
            app.swap_buffers();
        }
        _ => {}
    });
}
