extern crate kettlewin;
use kettlewin::*;

fn main() {
    // Create a new application with default settings.
    let mut app = Application::new().build().unwrap();
    let _window = app.new_window().title("Log Events").build().unwrap();

    app.event_loop().run(move |event| match event {
        Event::Draw => {}
        Event::WindowCloseRequested { .. } => app.quit(),
        _ => {
            println!("{:?}", event);
        }
    });
}
