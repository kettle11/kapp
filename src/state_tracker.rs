use crate::{Event, Key, MouseButton};
use std::collections::HashMap;
use std::time::Duration;


// In the future this could be extended to track:
// * Window positions and status.
// * Window scale factors
// * Window color spaces

/// Tracks key and mouse input state based on events.
pub struct StateTracker {
    keys_down_since_last_frame: HashMap<Key, Duration>, // Key was pressed since the last draw event for any window.
    keys_pressed: HashMap<Key, Duration>,
    mouse_buttons_down_since_last_frame: HashMap<MouseButton, Duration>, // Mouse was pressed since the last draw event for any window.
    mouse_buttons_pressed: HashMap<MouseButton, Duration>,
    mouse_position: (f32, f32),
}

impl StateTracker {
    pub fn new() -> Self {
        Self {
            keys_down_since_last_frame: HashMap::with_capacity(256), // Arbitrary numbers to avoid resize
            keys_pressed: HashMap::with_capacity(256),
            mouse_buttons_down_since_last_frame: HashMap::with_capacity(16),
            mouse_buttons_pressed: HashMap::with_capacity(16),
            mouse_position: (0., 0.),
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyDown { key, timestamp } => {
                self.keys_pressed.insert(key, timestamp);
                self.keys_down_since_last_frame.insert(key, timestamp);
            }
            Event::KeyUp { key, .. } => {
                self.keys_pressed.remove(&key);
            }
            Event::MouseButtonDown {
                button, timestamp, ..
            } => {
                self.mouse_buttons_pressed.insert(button, timestamp);
                self.mouse_buttons_down_since_last_frame
                    .insert(button, timestamp);
            }
            Event::MouseButtonUp { button, .. } => {
                self.mouse_buttons_pressed.remove(&button);
            }
            Event::MouseMoved { x, y, .. } => self.mouse_position = (x, y),
            _ => {}
        };
    }

    /// Occurs after the program event handler
    pub fn post_program_callback(&mut self, event: Event) {
        match event {
            Event::Draw { .. } => {
                self.mouse_buttons_down_since_last_frame.clear();
                self.keys_down_since_last_frame.clear();
            }
            _ => {}
        };
    }

    /// Returns true if the key has been pressed since the last draw
    pub fn key_down(&self, key: Key) -> bool {
        self.keys_down_since_last_frame.contains_key(&key)
    }

    /// Returns true if all the keys specified been pressed since the last draw.
    /// Right now this doesn't work perfectly for keyboard shortcuts because
    /// the different modifier keys are split out into their left and right versions.
    pub fn keys_down(&self, keys: &[Key]) -> bool {
        let mut pressed = true;
        for key in keys {
            pressed |= self.keys_down_since_last_frame.contains_key(&key)
        }
        pressed
    }

    /// Returns if the key is currently down
    pub fn key(&self, key: Key) -> bool {
        self.keys_pressed.contains_key(&key)
    }

    /// Returns true if the mouse button has been pressed since the last draw
    pub fn mouse_button_down(&self, button: MouseButton) -> bool {
        self.mouse_buttons_down_since_last_frame
            .contains_key(&button)
    }

    /// Returns true if the mouse button is pressed
    pub fn mouse_button(&self, button: MouseButton) -> bool {
        self.mouse_buttons_pressed.contains_key(&button)
    }

    pub fn mouse_position(&self) -> (f32, f32) {
        self.mouse_position
    }
}
