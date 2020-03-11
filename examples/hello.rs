extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut app = App::new(&AppParameters::default()).unwrap();
    let window = app
        .new_window(&WindowParameters {
            title: Some("Hello"),
            ..Default::default()
        })
        .unwrap();

    let gl_context = GLContext::new(); // Create a gl_context for the app
    let gl = gl_context.glow_context(); // Create a glow gl context for gl calls.

    gl_context.set_window(&window);

    // Run forever
    let mut color = 0.0;

    app.run(move |event, app| match event {
        Event::ButtonDown { .. } => {
            app.new_window(&WindowParameters {
                title: Some("Hello1"),
                ..Default::default()
            });
        }
        Event::Draw => {
            unsafe {
                gl.clear_color(0.0, color, 1.0, 1.0);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            }
            color += 0.01;

            gl_context.swap_buffers(); // Swaps the currently bound window.
                                       // Blocks if Vsync is used.

            // app.request_frame();
        }
        _ => {}
    });
}
