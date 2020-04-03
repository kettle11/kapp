use kettlewin::{Event::*, *};

fn main() {
    // Create a new window manager with default settings.
    let (mut app, mut event_loop) = initialize();
    let _window = app.new_window().build().unwrap();

    // Run forever
    event_loop.run(move |event| match event {
        WindowCloseRequested { .. } => app.quit(),
        KeyDown { key } => println!("Key pressed: {:?}", key),
        _ => println!("Received event: {:?}", event),
    });
}
