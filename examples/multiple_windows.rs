extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;
use std::time::{Duration, Instant};

fn main() {
    // Create a new window manager with default settings.
    let mut app = Application::new().build().unwrap();

    // Each of these GLContexts has their own separate resources.
    // They are separate contexts for interacting with OpenGL,
    // just like how seperate programs have separate contexts for OpenGL interaction.
    let mut gl_context = GLContext::new().build().unwrap();
    //let mut gl_context_red = GLContext::new().build().unwrap();

    let gl = gl_context.glow_context(); // Create a glow gl context for gl calls.
    let window_red = app
        .new_window()
        .position(200, 200)
        .title("Window Red")
        .build()
        .unwrap();
    let window_blue = app
        .new_window()
        .position(400, 400)
        .title("Window Blue")
        .build()
        .unwrap();

    //gl_context_blue.set_window(Some(&window_blue)).unwrap();
    //gl_context_red.set_window(Some(&window_red)).unwrap();

    let mut window_red = Some(window_red);
    let mut window_blue = Some(window_blue);

    gl_context.make_current();

    let mut now = Instant::now();

    // This method of multi-window rendering is perhaps suboptimal with vSync.
    // It's unclear how bad it is and more investigation is needed.
    // Additionally using two contexts means each context has different resources.
    // What approaches could be used for sharing?
    app.event_loop().run(move |event| match event {
        Event::WindowCloseRequested { window_id } => {
            if let Some(window) = window_red.as_ref() {
                if window.id == window_id {
                    window_red.take();
                    // The gl_context holds a reference to the window preventing it from being dropped.
                    gl_context.set_window(None).unwrap();
                }
            }
            if let Some(window) = window_blue.as_ref() {
                if window.id == window_id {
                    window_blue.take();
                    // The gl_context holds a reference to the window preventing it from being dropped.
                    gl_context.set_window(None).unwrap();
                }
            }
        }
        Event::Draw => {
            if window_red.is_some() {
                gl_context.set_window(window_red.as_ref()).unwrap();
                unsafe {
                    gl.clear_color(1.0, 0.0, 0.0, 1.0);
                    gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                    gl_context.swap_buffers();
                }
            }

            if window_blue.is_some() {
                gl_context.set_window(window_blue.as_ref()).unwrap();
                unsafe {
                    gl.clear_color(0.0, 0.0, 1.0, 1.0);
                    gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                    gl_context.swap_buffers();
                }
            }

            println!("{}", now.elapsed().as_millis());
            now = Instant::now();
            app.request_frame();
        }
        _ => {}
    });
}
