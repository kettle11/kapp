use kettlewin::*;
use ogl33::*;

fn main() {
    // Create a new window manager with default settings.
    let mut app = Application::new().build().unwrap();
    let window = app.new_window().title("OGL Example").build().unwrap();
    let gl_context = GLContext::new().build().unwrap(); // Create a gl_context for the app
    gl_context.set_window(&window).unwrap();

    unsafe {
        ogl33::load_gl_with(gl_context.gl_loader_c_string());
    }

    // Run forever
    app.event_loop().run(move |event| match event {
        Event::Draw => {
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
