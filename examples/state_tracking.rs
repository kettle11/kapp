/// Just display a window
use kapp::*;

fn main() {
    let (app, event_loop) = initialize();
    let _window = app.new_window().build().unwrap();

    event_loop.run(move |_event| {
        if app.mouse_button(MouseButton::Left) {
            println!("Mouse pressed");
        }
    });
}
