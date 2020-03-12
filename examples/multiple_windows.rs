extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut app = Application::new().build().unwrap();

    // Each of these GLContexts has their own separate resources.
    // They are separate contexts for interacting with OpenGL,
    // just like how seperate programs have separate contexts for OpenGL interaction.
    let gl_context_blue = GLContext::new().build().unwrap();
    let gl_context_red = GLContext::new().build().unwrap();

    let gl = gl_context_blue.glow_context(); // Create a glow gl context for gl calls.
    let window_red = app.new_window().title("Window Red").build().unwrap();
    let window_blue = app.new_window().title("Window Blue").build().unwrap();

    gl_context_blue.set_window(&window_blue).unwrap();
    gl_context_red.set_window(&window_red).unwrap();

    // This method of multi-window rendering is perhaps suboptimal with vSync.
    // It's unclear how bad it is and more investigation is needed.
    // Additionally using two contexts means each context has different resources.
    // What approaches could be used for sharing?
    app.event_loop().run(move |event| match event {
        Event::MouseMoved { .. } => {
            println!("Mouse moved: {:?}", event);
        }
        Event::Draw => {
            gl_context_red.make_current();
            unsafe {
                gl.clear_color(1.0, 0.0, 0.0, 1.0);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            }
            gl_context_blue.make_current();
            unsafe {
                gl.clear_color(0.0, 0.0, 1.0, 1.0);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            }
            gl_context_red.swap_buffers();
            gl_context_blue.swap_buffers();
        }
        _ => {}
    });
}
