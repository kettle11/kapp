extern crate gl;
extern crate kettlewin;
use kettlewin::*;

fn main() {
    // Create a new window manager with default settings.
    let mut window_manager = WindowManager::new().unwrap();
    // The 'true' here indicates that we want to panic if there are any errors while constructing the OpenGL context.
    let opengl_context = window_manager.new_opengl_context(true).unwrap();
}
