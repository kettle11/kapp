use crate::platform::*;

/// A handle used to control a Window.
/// The window is closed when the Window instance is dropped.
#[derive(Clone)]
pub struct Window {
    pub id: WindowId,
    platform_channel: PlatformChannel,
}

impl Window {
    pub fn new(id: WindowId, platform_channel: PlatformChannel) -> Self {
        Self {
            id,
            platform_channel,
        }
    }

    pub fn minimize(&mut self) {
        self.platform_channel
            .send(ApplicationMessage::MinimizeWindow { window: self.id });
    }

    pub fn maximize(&mut self) {
        self.platform_channel
            .send(ApplicationMessage::MaximizeWindow { window: self.id });
    }

    /// Returns the window from a minimized, maximized, or fullscreened state.
    pub fn restore(&mut self) {
        self.platform_channel
            .send(ApplicationMessage::RestoreWindow { window: self.id });
    }

    /// On Web this must be done in response to a user event.
    pub fn fullscreen(&mut self) {
        self.platform_channel
            .send(ApplicationMessage::FullscreenWindow { window: self.id });
    }

    /// Sets the title displayed at the top of the window
    pub fn set_title(&mut self, title: &str) {
        self.platform_channel
            .send(ApplicationMessage::SetWindowTitle {
                window: self.id,
                title: title.to_string(),
            });
    }
    /// Set the lower left corner of the window.
    pub fn set_position(&mut self, x: u32, y: u32) {
        self.platform_channel
            .send(ApplicationMessage::SetWindowPosition {
                window: self.id,
                x,
                y,
            });
    }

    /// Set the window's width and height, excluding the titlebar
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.platform_channel
            .send(ApplicationMessage::SetWindowSize {
                window: self.id,
                width,
                height,
            });
    }

    pub fn request_redraw(&mut self) {
        self.platform_channel
            .send(ApplicationMessage::RequestFrame { window: self.id });
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.platform_channel
            .send(ApplicationMessage::DropWindow { window: self.id });
        println!("Dropping window");
    }
}
