/// Just display a window
use kettlewin::*;

fn main() {
    let (mut app, mut event_loop) = initialize();
    let _window = app.new_window().build();

    event_loop.run(move |event| match event {
        Event::WindowCloseRequested { .. } => app.quit(),
        _ => {}
    });
}
