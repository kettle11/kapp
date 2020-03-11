extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

fn main() {
    let mut app = Application::new().build().unwrap();
    app.run_async(run);
}

async fn run(mut app: Application, mut events: Events) {
    // Create a new window manager with default settings.
    let window = app.new_window().title("Hello").build().unwrap();

    let gl_context = GLContext::new().build().unwrap(); // Create a gl_context for the app
    let gl = gl_context.glow_context(); // Create a glow gl context for gl calls.

    gl_context.set_window(&window).unwrap();

    // Run forever
    let mut color = 0.0;

    // Loop forever!
    loop {
        match events.next_event().await {
            Event::Draw => {
                unsafe {
                    gl.clear_color(0.0, 0.0, color, 1.0);
                    gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                }
                color += 0.02;

                gl_context.swap_buffers(); // Swaps the currently bound window.
                                           // Blocks if Vsync is used.
            }
            _ => {}
        }
    }
}
