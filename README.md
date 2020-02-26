# kettlewin
A pure Rust window and input library for Windows. (Don't use it yet) 
This project is in the very early stages and is missing most features, is untested, and the code isn't great.
That said, it has very few dependencies and builds extremely quickly. 

## Example
```rust
extern crate kettlewin;
use kettlewin::glow::*;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut window_manager = WindowManager::new().build().unwrap();
    let gl = window_manager.gl_context();
    let window = window_manager.new_window().build().unwrap();

    // Run forever
    run(move |event| match event {
        Event::Draw => {
            unsafe {
                gl.clear_color(0.0, 1.0, 1.0, 1.0);
                gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            }
            // When we're done rendering swap the window buffers to display to the screen.
            window_manager.swap_buffers(&window);
        }
        _ => {}
    });
}
```

## Features
* Create multiple windows with a single shared OpenGL context
* Mouse input
* Keyboard input
* Specify backbuffer properties
