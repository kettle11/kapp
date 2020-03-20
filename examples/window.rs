/// Just display a window
use kettlewin::Event::*;
use kettlewin::*;

fn main() {
    let mut app = Application::new();
    let _window = app.new_window().build();

    app.run(|app, event| match event {
        WindowCloseRequested { .. } => app.quit(),
        _ => {}
    });
}
