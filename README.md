# kettlewin
A pure Rust window and input library for MacOS and Web.

Strives to be unsurprising, quick to build, and straightforward to maintain.

Kettlewin is a work in progress. There are rough edges, unimplemented functions, and many bugs.

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
