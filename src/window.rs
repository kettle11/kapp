use crate::platform::*;
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

    /// Returns the window from a minimized or maximized state.
    pub fn restore(&mut self) {
        self.platform_channel
            .send(ApplicationMessage::RestoreWindow { window: self.id });
    }

    pub fn fullscreen(&mut self) {
        self.platform_channel
            .send(ApplicationMessage::FullscreenWindow { window: self.id });
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
}

impl Drop for Window {
    fn drop(&mut self) {
        self.platform_channel
            .send(ApplicationMessage::DropWindow { window: self.id });
        println!("Dropping window");
    }
}
