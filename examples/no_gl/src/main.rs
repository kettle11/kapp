extern crate kettlewin;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut app = Application::new();
    let window = app.new_window().build().unwrap();

    // Run forever
    app.run(move |app, event| match event {
        Event::WindowCloseRequested { .. } => app.quit(),
        _ => {}
    });
}
