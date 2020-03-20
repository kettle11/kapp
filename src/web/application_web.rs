use crate::application_message::{ApplicationMessage, ApplicationMessage::*};
use crate::{Application, Event};
use std::sync::mpsc::*;

pub fn process_events(event_receiver: Receiver<ApplicationMessage>) {
    while let Ok(event) = event_receiver.try_recv() {
        match event {
            MinimizeWindow { window } => {}
            SetWindowPosition { window, x, y } => {}
            SetWindowSize {
                window,
                width,
                height,
            } => {}
            MaximizeWindow { .. } => {}
            FullscreenWindow { window } => {}
            RestoreWindow { .. } => unimplemented!(),
            DropWindow { .. } => unimplemented!(),
            RequestFrame { .. } => {}
            SetMousePosition { x, y } => {}
            Quit => {}
            NewWindow {
                window_parameters,
                response_channel,
            } => {}
        }
    }
}

pub struct PlatformApplication {}

impl PlatformApplication {
    /// Only call from the main thread.
    pub fn new(
        program_to_application_receive: Receiver<crate::application_message::ApplicationMessage>,
    ) -> Self {
        Self {}
    }

    /// Only call from the main thread.
    pub fn flush_events(&mut self) {}

    /// Only call from the main thread.
    pub fn start_receiver<T>(
        &self,
        mut application: crate::Application,
        mut callback: T,
        receive_channel: Receiver<crate::Event>,
    ) where
        T: 'static + FnMut(&mut Application, crate::Event),
    {
    }

    /// Only call from the main thread.
    pub fn start_application(self, send_channel: Sender<crate::Event>) {
        
    }

    pub fn get_waker(&self) -> PlatformApplicationWaker {
        PlatformApplicationWaker {}
    }
}

#[derive(Clone)]
pub struct PlatformApplicationWaker {}

impl PlatformApplicationWaker {
    /// Call from any thread
    pub fn wake(&self) {}
}
