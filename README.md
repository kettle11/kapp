# kApp

kApp is a pure Rust window and input library for macOS, Web, and Windows.

kApp strives to be unsurprising, quick to build, and straightforward to maintain.

A clean build of kApp on macOS takes  about 3.5 seconds.

**kApp is a work in progress.**

I am improving kApp slowly and steadily as issues come up. It is usable as is, but some functionality is missing and everything is subject to change. If you try it out and run into a problem open an issue and please consider contributing!

Currently, to keep the scope manageable, I've only been working to support the latest versions of each OS. My goal has been to get kapp to a point of consistency and quality across the latest versions of Windows/macOS as well as modern browsers before evaluating other platforms. Adding Linux support is also a goal for the future, though currently I am unable to work on it. It's an area where I'd very much welcome contributors and collaboration.

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
