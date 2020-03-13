extern crate kettlewin;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut app = Application::new().build().unwrap();
    let _window = app.new_window().build().unwrap();

    // Run forever
    app.event_loop().run(move |event| match event {
        _ => {}
    });
}
