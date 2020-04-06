# kettlewin
A pure Rust window and input library for MacOS and Web.

## Example
```rust
use kettlewin::*;

fn main() {
    let (mut app, mut event_loop) = initialize();
    let _window = app.new_window().build();

    event_loop.run(move |event| match event {
        Event::WindowCloseRequested { .. } => app.quit(),
        Event::Draw { .. } => {
            // Render something here.
        }
        _ => {}
    });
}
```

## Features
* Create multiple windows
* Mouse input
* Keyboard input
* OpenGL context creation
