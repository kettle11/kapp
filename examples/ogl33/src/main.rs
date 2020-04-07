use kettlewin::*;
use ogl33::*;

fn main() {
    let (mut app, mut event_loop) = initialize();
    let window = app.new_window().title("OGL33 Example").build().unwrap();
    let mut gl_context = GLContext::new().build().unwrap(); // Create a gl_context for the app
    gl_context.set_window(Some(&window.id)).unwrap();

    unsafe {
        ogl33::load_gl_with(gl_context.gl_loader_c_string());
    }

    // Run forever
    event_loop.run(move |event| match event {
        Event::Draw { .. } => {
            // gl_context.make_current();
            unsafe {
                glClearColor(0.0, 1.0, 1.0, 1.0);
                glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            }
            // When we're done rendering swap the window buffers to display to the screen.
            gl_context.swap_buffers();
        }
        _ => {}
    });
}
