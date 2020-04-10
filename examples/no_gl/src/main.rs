use kettlewin::{Event::*, *};

fn main() {
    // Create a new window manager with default settings.
    let (app, event_loop) = initialize();
    let window = app.new_window().build().unwrap();

    // Run forever
    event_loop.run(move |event| match event {
        WindowCloseRequested { .. } => app.quit(),
        EventsCleared => {
            println!("here");
            std::thread::sleep(std::time::Duration::from_millis(16));
            window.request_redraw();
        }
        KeyDown { key } => println!("Key pressed: {:?}", key),
        _ => println!("Received event: {:?}", event),
    });
}
