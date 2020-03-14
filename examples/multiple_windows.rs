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

    let mut window_red = Some(window_red);
    let mut window_blue = Some(window_blue);

    // This method of multi-window rendering is perhaps suboptimal with vSync.
    // It's unclear how bad it is and more investigation is needed.
    // Additionally using two contexts means each context has different resources.
    // What approaches could be used for sharing?
    app.event_loop().run(move |event| match event {
        Event::WindowCloseRequested { window_id } => {
            if let Some(window) = window_red.as_ref() {
                if window.id == window_id {
                    window_red.take();
                }
            }
            if let Some(window) = window_blue.as_ref() {
                if window.id == window.id {
                    window_blue.take();
                }
            }
        }
        Event::ButtonDown { button } => match button {
            Button::Q => {
                app.quit();
            }
            _ => {}
        },
        Event::Draw => {
            if let Some(window_red) = window_red.as_ref() {
                gl_context_red.make_current();
                unsafe {
                    gl.clear_color(1.0, 0.0, 0.0, 1.0);
                    gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                    gl_context_red.swap_buffers();
                }
            }

            if let Some(window_blue) = window_blue.as_ref() {
                gl_context_blue.make_current();
                unsafe {
                    gl.clear_color(0.0, 0.0, 1.0, 1.0);
                    gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                    gl_context_blue.swap_buffers();
                }
            }
        }
        _ => {}
    });
}
