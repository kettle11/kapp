use crate::platform::*;
use crate::{Application, Window};

pub struct WindowBuilder<'a> {
    application: &'a mut Application,
    window_parameters: WindowParameters,
}

impl<'a> WindowBuilder<'a> {
    pub fn new(application: &'a mut Application) -> Self {
        Self {
            application,
            window_parameters: WindowParameters {
                position: None,
                dimensions: None,
                resizable: true,
                title: None,
            },
        }
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.window_parameters.title = Some(title.to_string());
        self
    }

    pub fn resizable(&mut self, resizable: bool) -> &mut Self {
        self.window_parameters.resizable = resizable;
        self
    }

    /// Specifies the lower left corner of the window.
    pub fn position(&mut self, x: u32, y: u32) -> &mut Self {
        self.window_parameters.position = Some((x, y));
        self
    }

    pub fn dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.window_parameters.dimensions = Some((width, height));
        self
    }

    // Web doesn't require any of this more complex window building behavior.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn build(&mut self) -> Result<Window, ()> {
        Ok(Window::new(
            self.application
                .platform_application
                .borrow_mut()
                .new_window(&self.window_parameters),
            self.application.platform_application.clone(),
        ))
    }

    #[cfg(target_arch = "wasm32")]
    pub fn build(&mut self) -> Result<Window, ()> {
        Ok(Window::new(
            crate::WindowId::new(0 as *mut std::ffi::c_void),
            self._application.platform_channel.clone(),
        ))
    }
}
