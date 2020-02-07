# kettlewin
A pure Rust window and input library for Windows. (Don't use it yet) 

This project is in the very early stages and is missing most features, is untested, and the code isn't great.

That said, it has very few dependencies and builds extremely quickly. 

## Features
* Create multiple windows with a shared OpenGL context
* Mouse input
* Keyboard input

## Example
```rust
extern crate gl;
extern crate kettlewin;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut window_manager = WindowManager::new().unwrap();
    gl::load_with(window_manager.gl_loader());
    let window = window_manager.new_window().build().unwrap();

    // Run forever
    run(move |event| match event {
        Event::Draw => {
            unsafe {
                gl::ClearColor(0.0, 1.0, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }
            // When we're done rendering swap the window buffers to display to the screen.
            window_manager.swap_buffers(&window);
        }
        _ => {}
    });
}

```
