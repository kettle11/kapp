use crate::application_message::{ApplicationMessage, ApplicationMessage::*};
use crate::WindowId;
use std::sync::mpsc;

#[derive(Debug, Clone)]
pub struct Window {
    pub id: WindowId,
    message_channel: mpsc::Sender<ApplicationMessage>,
}

impl Window {
    pub fn new(id: WindowId, message_channel: mpsc::Sender<ApplicationMessage>) -> Self {
        Self {
            id,
            message_channel,
        }
    }

    pub fn minimize(&self) {
        self.message_channel
            .send(MinimizeWindow { window: self.id })
            .unwrap();
    }

    pub fn maximize(&self) {}

    /// Returns the window from a minimized or maximized state.
    pub fn restore(&self) {
        self.message_channel
            .send(RestoreWindow { window: self.id })
            .unwrap();
    }

    pub fn fullscreen(&self) {
        self.message_channel
            .send(FullscreenWindow { window: self.id })
            .unwrap();
    }

    /// Set the lower left corner of the window.
    pub fn set_position(&self, x: u32, y: u32) {
        self.message_channel
            .send(SetWindowPosition {
                window: self.id,
                x,
                y,
            })
            .unwrap();
    }

    /// Set the window's width and height, excluding the titlebar
    pub fn set_size(&self, width: u32, height: u32) {
        self.message_channel
            .send(SetWindowSize {
                window: self.id,
                width,
                height,
            })
            .unwrap();
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.message_channel
            .send(DropWindow { window: self.id })
            .unwrap();
        println!("Dropping window");
    }
}
