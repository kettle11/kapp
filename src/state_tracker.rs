use crate::{Event, Key, PointerButton, PointerSource};
use std::collections::HashMap;
use std::time::Duration;

// In the future this could be extended to track:
// * Window positions and status.
// * Window scale factors
// * Window color spaces

/// Tracks key and mouse input state based on events.
pub struct StateTracker {
    keys_down_since_last_frame: HashMap<Key, Duration>, // Key was pressed since the last clear for any window.
    keys_pressed: HashMap<Key, Duration>,
    mouse_buttons_down_since_last_frame: HashMap<PointerButton, Duration>, // Mouse was pressed since the last clear for any window.
    mouse_buttons_pressed: HashMap<PointerButton, Duration>,
    mouse_position: (f64, f64),
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
            Event::PointerDown {
                button,
                source: PointerSource::Mouse,
                timestamp,
                ..
            } => {
                self.mouse_buttons_pressed.insert(button, timestamp);
                self.mouse_buttons_down_since_last_frame
                    .insert(button, timestamp);
            }
            Event::PointerUp {
                button,
                source: PointerSource::Mouse,
                ..
            } => {
                self.mouse_buttons_pressed.remove(&button);
            }
            Event::PointerMoved {
                x,
                y,
                source: PointerSource::Mouse,
                ..
            } => self.mouse_position = (x, y),
            _ => {}
        };
    }

    /// Reset any "button down" states
    pub fn clear(&mut self) {
        self.mouse_buttons_down_since_last_frame.clear();
        self.keys_down_since_last_frame.clear();
    }

    /// Returns true if the key has been pressed since the last call to clear.
    pub fn key_down(&self, key: Key) -> bool {
        self.keys_down_since_last_frame.contains_key(&key)
    }

    /// Returns true if all the keys specified been pressed since the last call to clear.
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

    /// Returns true if the mouse button has been pressed since the last call to clear.
    pub fn mouse_button_down(&self, button: PointerButton) -> bool {
        self.mouse_buttons_down_since_last_frame
            .contains_key(&button)
    }

    /// Returns true if the mouse button is pressed
    pub fn mouse_button(&self, button: PointerButton) -> bool {
        self.mouse_buttons_pressed.contains_key(&button)
    }

    pub fn mouse_position(&self) -> (f64, f64) {
        self.mouse_position
    }
}
