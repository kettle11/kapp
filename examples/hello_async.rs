use glow::*;
use kettlewin::*;
fn main() {
    let (app, mut event_loop) = initialize();
    event_loop.run_async(app, run);
}

async fn run(mut app: Application, mut events: Events) {
    let mut window = app.new_window().build().unwrap();

    // Create a GLContext
    let mut gl_context = GLContext::new().build().unwrap();

    // Assign the GLContext's window.
    gl_context.set_window(Some(&window.id)).unwrap();

    // Glow is a library for accessing GL function calls from a variety of platforms
    // Glow requires a cross platform way to load function pointers,
    // which GLContext provides with get_proc_address.

    #[cfg(target_arch = "wasm32")]
    let gl = glow::Context::from_webgl1_context(gl_context.get_webgl1_context());
    #[cfg(not(target_arch = "wasm32"))]
    let gl = glow::Context::from_loader_function(|s| gl_context.get_proc_address(s));

    // Run forever
    let mut color = 0.0;

    // Loop forever!
    loop {
        match events.next_event().await {
            Event::WindowCloseRequested { .. } => app.quit(),
            Event::Draw { .. } => {
                unsafe {
                    gl.clear_color(1.0, 1.0, color, 1.0);
                    gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                }
                color += 0.01;

                gl_context.swap_buffers(); // Swaps the currently bound window.
                window.request_redraw();
            }
            _ => {}
        }
    }
}
