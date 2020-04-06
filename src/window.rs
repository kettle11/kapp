use crate::platform::*;
use std::cell::RefCell;
use std::rc::Rc;

/// A handle used to control a Window.
/// The window is closed when the Window instance is dropped.
#[derive(Clone)]
pub struct Window {
    pub id: WindowId,
    platform_application: Rc<RefCell<PlatformApplication>>,
}

impl Window {
    pub(crate) fn new(
        id: WindowId,
        platform_application: Rc<RefCell<PlatformApplication>>,
    ) -> Self {
        Self {
            id,
            platform_application,
        }
    }

    pub fn minimize(&mut self) {
        self.platform_application
            .borrow_mut()
            .minimize_window(self.id);
    }

    pub fn maximize(&mut self) {
        self.platform_application
            .borrow_mut()
            .maximize_window(self.id);
    }

    /// Returns the window from a minimized, maximized, or fullscreened state.
    pub fn restore(&mut self) {
        self.platform_application
            .borrow_mut()
            .restore_window(self.id);
    }

    /// On Web this must be done in response to a user event.
    pub fn fullscreen(&mut self) {
        self.platform_application
            .borrow_mut()
            .fullscreen_window(self.id);
    }

    /// Sets the title displayed at the top of the window
    pub fn set_title(&mut self, title: &str) {
        self.platform_application
            .borrow_mut()
            .set_window_title(self.id, title);
    }
    /// Set the lower left corner of the window.
    pub fn set_position(&mut self, x: u32, y: u32) {
        self.platform_application
            .borrow_mut()
            .set_window_position(self.id, x, y);
    }

    /// Set the window's width and height, excluding the titlebar
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.platform_application
            .borrow_mut()
            .set_window_dimensions(self.id, width, height);
    }

    pub fn request_redraw(&mut self) {
        self.platform_application
            .borrow_mut()
            .redraw_window(self.id);
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.platform_application
            .borrow_mut()
            .close_window(self.id);
        println!("Dropping window");
    }
}
