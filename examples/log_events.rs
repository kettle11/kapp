extern crate kettlewin;
use kettlewin::*;

fn main() {
    // Create a new application with default settings.
    let mut app = Application::new();
    let _window = app.new_window().title("Log Events").build().unwrap();

    app.run(move |app, event| match event {
        Event::Draw => {}
        Event::WindowCloseRequested { .. } => app.quit(),
        _ => {
            println!("{:?}", event);
        }
    });
}
