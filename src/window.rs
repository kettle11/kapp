use crate::application_message::ApplicationMessage::*;
use crate::platform_traits::PlatformChannelTrait;
use crate::PlatformChannel;
use crate::WindowId;

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
            .send(MinimizeWindow { window: self.id });
    }

    pub fn maximize(&mut self) {}

    /// Returns the window from a minimized or maximized state.
    pub fn restore(&mut self) {
        self.platform_channel
            .send(RestoreWindow { window: self.id });
    }

    pub fn fullscreen(&mut self) {
        self.platform_channel
            .send(FullscreenWindow { window: self.id });
    }

    /// Set the lower left corner of the window.
    pub fn set_position(&mut self, x: u32, y: u32) {
        self.platform_channel.send(SetWindowPosition {
            window: self.id,
            x,
            y,
        });
    }

    /// Set the window's width and height, excluding the titlebar
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.platform_channel.send(SetWindowSize {
            window: self.id,
            width,
            height,
        });
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.platform_channel.send(DropWindow { window: self.id });
        println!("Dropping window");
    }
}
