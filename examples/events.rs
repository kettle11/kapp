extern crate kettlewin;
use kettlewin::*;

fn main() {
    let (app, event_loop) = initialize();
    let _window = app.new_window().title("Log Events").build().unwrap();

    event_loop.run(move |event| match event {
        // Event::TrackpadTouch { .. } => {}
        Event::Draw { .. } => {}
        Event::WindowCloseRequested { .. } => app.quit(),
        _ => {
            println!("{:?}", event);
        }
    });
}
