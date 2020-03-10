extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

// This example just displays a full red window.
// It's used to verify assumptions about a display's color space.
fn main() {
    // Create a new window manager with default settings.
    let mut app = App::new().build().unwrap();
    let gl = app.gl_context();
    let _window = app.new_window().title("Red").build(&app).unwrap();

    app.run(move |event, app| match event {
        Event::Draw => {
            unsafe {
                gl.clear_color(1.0, 0.0, 0.0, 1.0);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            }
            // When we're done rendering swap the window buffers to display to the screen.
            app.swap_buffers();

            // app.request_frame();
        }
        _ => {}
    });
}
