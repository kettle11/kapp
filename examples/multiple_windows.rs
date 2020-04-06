use glow::*;
use kettlewin::*;
use std::time::Instant;

fn main() {
    // Create a new window manager with default settings.
    let (mut app, mut event_loop) = initialize();

    // Create a GLContext
    let mut gl_context = GLContext::new().build().unwrap();
    let gl = glow::Context::from_loader_function(|s| gl_context.get_proc_address(s));

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
    event_loop.run(move |event| match event {
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
        Event::Draw { window_id } => {
            gl_context.make_current(); // Why is this needed?
            if let Some(window_red) = window_red.as_mut() {
                if window_red.id == window_id {
                    gl_context.set_window(Some(&window_red.id)).unwrap();
                    unsafe {
                        gl.clear_color(1.0, 0.0, 0.0, 1.0);
                        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                        gl_context.swap_buffers();
                    }
                    window_red.request_redraw();
                }
            }

            if let Some(window_blue) = window_blue.as_mut() {
                if window_blue.id == window_id {
                    gl_context.set_window(Some(&window_blue.id)).unwrap();
                    unsafe {
                        gl.clear_color(0.0, 0.0, 1.0, 1.0);
                        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                        gl_context.swap_buffers();
                    }
                    window_blue.request_redraw();
                }
            }

            println!("{}", now.elapsed().as_millis());
            now = Instant::now();
        }
        _ => {}
    });
}
