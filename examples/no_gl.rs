extern crate kettlewin;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut app = App::new().build().unwrap();
    let _window = app.new_window().build(&app).unwrap();

    // Run forever
    app.run(move |event, app| match event {
        Event::Draw => {
            // When we're done rendering swap the window buffers to display to the screen.
            app.swap_buffers();
        }
        _ => {}
    });
}
