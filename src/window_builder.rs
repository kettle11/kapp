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
                title: "Untitled".to_string(),
            },
        }
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.window_parameters.title = title.to_string();
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

    pub fn build(&mut self) -> Result<Window, ()> {
        Ok(Window::new(
            self.application
                .platform_application
                .borrow_mut()
                .new_window(&self.window_parameters),
            self.application.platform_application.clone(),
        ))
    }
}
