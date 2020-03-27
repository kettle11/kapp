use glow::*;
use kettlewin::*;

fn main() {
    let (mut app, event_loop) = initialize();
    let mut window = app.new_window().build().unwrap();

    // Create a GLContext
    let mut gl_context = GLContext::new().build().unwrap();

    // Assign the GLContext's window.
    gl_context.set_window(Some(&window)).unwrap();

    // Glow is a library for accessing GL function calls from a variety of platforms
    // Glow requires a cross platform way to load function pointers,
    // which GLContext provides with get_proc_address.
    let gl = glow::Context::from_loader_function(|s| gl_context.get_proc_address(s));

    event_loop.run(move |event| match event {
        Event::Draw => {
            // Make the GLContext current to the thread that this callback runs on.
            gl_context.make_current();

            // Clear the screen to a lovely shade of blue.
            unsafe {
                gl.clear_color(0.3765, 0.3137, 0.8627, 1.0);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            }
            // Finally display what we've drawn.
            gl_context.swap_buffers();

            // It is not necessary for this example,
            // but calling request_frame ensures the program redraws continuously.
            //app.request_frame();
        }
        _ => {}
    });
}
