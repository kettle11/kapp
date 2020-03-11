extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

fn main() {
    let mut app = App::new(&AppParameters::default()).unwrap();
    app.run_async(run);
}

async fn run(mut app: App, mut events: Events) {
    let window = app
        .new_window(&WindowParameters {
            title: Some("Hello Async"),
            ..Default::default()
        })
        .unwrap();

    let gl_context = GLContext::new(); // Create a gl_context for the app
    let gl = gl_context.glow_context(); // Create a glow gl context for gl calls.

    gl_context.set_window(&window).unwrap();

    let mut color = 0.0;

    // Loop forever!
    loop {
        match events.next_event().await {
            Event::Draw => {
                unsafe {
                    gl.clear_color(0.0, color, 1.0, 1.0);
                    gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                }
                color += 0.01;

                gl_context.swap_buffers(); // Swaps the currently bound window.
                                           // Blocks if Vsync is used.
            }
            _ => {}
        }
    }
}
