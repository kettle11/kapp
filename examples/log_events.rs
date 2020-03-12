extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

fn main() {
    // Create a new application with default settings.
    let mut app = Application::new().build().unwrap();
    let window = app.new_window().title("Log Events").build().unwrap();

    app.event_loop().run(move |event| {
        println!("Event: {:?}", event);
    });
}
