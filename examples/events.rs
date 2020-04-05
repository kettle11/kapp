extern crate kettlewin;
use kettlewin::*;

fn main() {
    // Create a new application with default settings.
    let (mut app, mut event_loop) = initialize();
    let _window = app.new_window().title("Log Events").build().unwrap();

    event_loop.run(move |event| match event {
        Event::TrackpadTouch { .. } => {}
        Event::Draw { .. } => {}
        Event::WindowCloseRequested { .. } => app.quit(),
        _ => {
            println!("{:?}", event);
        }
    });
}
