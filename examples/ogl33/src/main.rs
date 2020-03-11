use kettlewin::*;
use ogl33::*;

fn main() {
    // Create a new window manager with default settings.
    let mut app = App::new(&AppParameters::default()).unwrap();
    let _window = app
        .new_window(&WindowParameters {
            title: Some("Hello"),
            ..Default::default()
        })
        .unwrap();
    unsafe {
        load_gl_with(app.gl_loader_c_string());
    }

    let mut dims = [0 as GLint; 4];
    unsafe {
        glGetIntegerv( GL_VIEWPORT, (&mut dims[0]) as *mut GLint);
    }

    unsafe {
        glClearColor(1.0, 1.0, 1.0, 1.0);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        glEnable(GL_SCISSOR_TEST);

    }

    println!("Dimensions: {:?}", dims);
    // Run forever
    app.run(move |event, app| match event {
        Event::Draw => {
            unsafe {
                glDisable(GL_SCISSOR_TEST);
                glClearColor(1.0, 1.0, 1.0, 1.0);
                glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

                glEnable(GL_SCISSOR_TEST);

                glScissor(20, 20, 23 as i32, 23 as i32);
                glClearColor(0.0, 0.0, 0.0, 1.0);
                glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    
            }

            let mut dims = [0 as GLint; 4];
            unsafe {
                glGetIntegerv( GL_VIEWPORT, (&mut dims[0]) as *mut GLint);
            }
        
            println!("Dimensions: {:?}", dims);

            // When we're done rendering swap the window buffers to display to the screen.
            app.swap_buffers();
        }
        _ => {}
    });
}
