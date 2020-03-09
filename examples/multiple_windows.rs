extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut app = App::new().build().unwrap();
    let gl = app.gl_context();

    let window_red = app.new_window().title("Window Red").build(&app).unwrap();
    let window_blue = app.new_window().title("Window Blue").build(&app).unwrap();

    // Run forever
    app.run(move |event, app| match event {
        Event::Draw => {
            draw_to_window(&gl, &app, &window_red, 1.0, 0.0, 0.0);
            draw_to_window(&gl, &app, &window_blue, 0.0, 0.0, 1.0);
            swap_window(&app, &window_red);
            swap_window(&app, &window_blue);
        }
        _ => {}
    });
}

fn draw_to_window(gl: &Context, app: &App, window: &Window, r: f32, g: f32, b: f32) {
    // If make_current fails the window may no longer be open.
    if app.make_current(window).is_ok() {
        unsafe {
            gl.clear_color(r, g, b, 1.0);
            gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        }
        // When we're done rendering swap the window buffers to display to the screen.
    }
}

fn swap_window(app: &App, window: &Window) {
    if app.make_current(window).is_ok() {
        app.swap_buffers();
    }
}
