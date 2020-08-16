# kApp

kApp is a work in progress. There are rough edges, unimplemented functions, and many bugs.

kApp is a pure Rust window and input library for MacOS, Web, and Windows.

kApp strives to be unsurprising, quick to build, and straightforward to maintain.

A clean build of kApp on MacOS takes  about 3.5 seconds.

## Example

```rust
use kapp::*;

fn main() {
    let (app, event_loop) = initialize();
    let _window = app.new_window().build().unwrap();

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

* Create windows
* Mouse input
* Keyboard input
* Event timestamps

## Similar Projects

The following projects were valuable resources that inspired kApp.

[Winit](https://github.com/rust-windowing/winit)

[Makepad](https://github.com/makepad/makepad)

[Glutin](https://github.com/rust-windowing/glutin)

[SDL2](https://www.libsdl.org/download-2.0.php)

[Sokol](https://github.com/floooh/sokol)

[GLFW](https://www.glfw.org/)
