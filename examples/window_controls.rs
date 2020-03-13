extern crate kettlewin;
use kettlewin::*;

fn main() {
    // Create a new application with default settings.
    let mut app = Application::new().build().unwrap();
    let window = app.new_window().title("Window Controls").build().unwrap();

    app.event_loop().run(move |event| match event {
        Event::ButtonDown { button } => match button {
            Button::Digit1 => {
                window.set_position(0, 0);
            }
            Button::Digit2 => {
                window.set_position(400, 0);
            }
            Button::Digit3 => {
                window.set_position(800, 0);
            }
            Button::A => {
                window.set_size(100, 100);
            }
            Button::S => {
                window.set_size(400, 400);
            }
            Button::D => {
                window.set_size(800, 800);
            }
            Button::M => {
                window.minimize();
            }
            Button::F => {
                window.fullscreen();
            }
            Button::R => {
                window.restore();
            }
            Button::C => {
                window.close();
            }
            Button::Q => {
                app.quit();
            }
            _ => {}
        },
        _ => {}
    });
}
