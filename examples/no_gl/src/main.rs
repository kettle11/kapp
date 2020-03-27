extern crate kettlewin;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let (mut app, event_loop) = initialize();
    let _window = app.new_window().build().unwrap();

    // Run forever
    event_loop.run(Box::new(move |event| match event {
        WindowCloseRequested { .. } => app.quit(),
        _ => {}
    }));
}
