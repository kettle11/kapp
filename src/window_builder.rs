use crate::platform_traits::PlatformApplicationTrait;
use crate::platform_traits::PlatformChannelTrait;
use crate::{application_message::ApplicationMessage, Application, PlatformApplication, Window};
use std::sync::mpsc;
pub struct WindowBuilder<'a> {
    application: &'a mut Application,
    platform_application: Option<&'a mut PlatformApplication>,
    window_parameters: WindowParameters,
}

#[derive(Clone)]
pub struct WindowParameters {
    pub position: Option<(u32, u32)>,
    pub dimensions: Option<(u32, u32)>,
    pub resizable: bool,
    pub title: Option<String>,
}

impl<'a> WindowBuilder<'a> {
    pub fn new(
        application: &'a mut Application,
        platform_application: Option<&'a mut PlatformApplication>,
    ) -> Self {
        Self {
            application,
            platform_application,
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

    pub fn build(&mut self) -> Result<Window, ()> {
        let (sender, receiver) = mpsc::channel();
        self.application
            .platform_channel
            .send(ApplicationMessage::NewWindow {
                window_parameters: self.window_parameters.clone(),
                response_channel: sender,
            });

        // If a PlatformApplication has been passed in then build is called from the main thread.
        // In that case the platform application events must be flushed.
        if let Some(platform_application) = self.platform_application.as_mut() {
            platform_application.flush_events();
        }
        let result = receiver.recv().unwrap();
        result.map(|id| Window::new(id, self.application.platform_channel.clone()))
    }
}
