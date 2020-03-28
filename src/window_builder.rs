use crate::platform::*;
use crate::{Application, Window};

pub struct WindowBuilder<'a> {
    _application: &'a mut Application,
    window_parameters: WindowParameters,
}

impl<'a> WindowBuilder<'a> {
    pub fn new(_application: &'a mut Application) -> Self {
        Self {
            _application,
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
        let (sender, receiver) = crate::platform::single_value_channel::channel();
        self._application
            .platform_channel
            .send(ApplicationMessage::NewWindow {
                window_parameters: self.window_parameters.clone(),
                response_channel: sender,
            });

        self._application.flush_events();
        let result = receiver.recv().unwrap();
        result.map(|id| Window::new(id, self._application.platform_channel.clone()))
    }

    #[cfg(target_arch = "wasm32")]
    pub fn build(&mut self) -> Result<Window, ()> {
        Ok(Window::new(
            crate::WindowId::new(0 as *mut std::ffi::c_void),
            self._application.platform_channel.clone(),
        ))
    }
}
