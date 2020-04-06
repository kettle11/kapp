extern crate kettlewin;
use kettlewin::*;

fn main() {
    // Create a new application with default settings.
    let (mut app, mut event_loop) = initialize();
    let mut window = app.new_window().title("Window Controls").build().unwrap();

    event_loop.run(move |event| match event {
        Event::KeyDown { key } => match key {
            Key::Digit1 => {
                window.set_position(0, 0);
            }
            Key::Digit2 => {
                window.set_position(400, 0);
            }
            Key::Digit3 => {
                window.set_position(800, 0);
            }
            Key::A => {
                window.set_size(100, 100);
            }
            Key::S => {
                window.set_size(400, 400);
            }
            Key::D => {
                window.set_size(800, 800);
            }
            Key::M => {
                window.minimize();
            }
            Key::F => {
                window.fullscreen();
            }
            Key::R => {
                window.restore();
            }
            Key::Q => {
                app.quit();
            }
            _ => {}
        },
        _ => {}
    });
}
