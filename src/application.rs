use crate::application_message::ApplicationMessage;
use crate::application_message::ApplicationMessage::*;
use crate::PlatformApplication;
use std::sync::mpsc::*;

pub struct Application {
    pub program_to_application_send: Sender<ApplicationMessage>,
    main_thread_id: Option<std::thread::ThreadId>,
    platform_application: Option<PlatformApplication>,
}

impl Application {
    pub fn new() -> Self {
        let (program_to_application_send, program_to_application_receive) =
            channel::<ApplicationMessage>();
        Self {
            platform_application: Some(PlatformApplication::new(program_to_application_receive)),
            program_to_application_send,
            main_thread_id: None,
        }
    }

    pub fn from_parts(
        program_to_application_send: Sender<ApplicationMessage>,
        main_thread_id: Option<std::thread::ThreadId>,
    ) -> Self {
        Self {
            program_to_application_send,
            main_thread_id,
            platform_application: None,
        }
    }

    pub fn to_parts(&self) -> (Sender<ApplicationMessage>, Option<std::thread::ThreadId>) {
        (
            self.program_to_application_send.clone(),
            self.main_thread_id.clone(),
        )
    }

    /// Only call this trom the main thread.
    /// This will never return.
    pub fn run<T>(&mut self, callback: T) -> !
    where
        T: 'static + FnMut(&mut Application, crate::Event) + Send,
    {
        if let Some(platform_application) = self.platform_application.take() {
            self.main_thread_id = Some(std::thread::current().id());

            // Platform specific code to run.
            platform_application.run(&self, callback);
            unreachable!()
        } else {
            panic!("Run can only be called once per program")
        }
    }

    pub fn on_main_thread(&self) -> bool {
        self.main_thread_id
            .map_or(false, |id| id == std::thread::current().id())
    }

    pub fn request_frame(&mut self) {
        self.program_to_application_send.send(RequestFrame).unwrap();
    }

    pub fn quit(&self) {
        self.program_to_application_send.send(Quit).unwrap();
    }

    pub fn set_mouse_position(&self, x: u32, y: u32) {
        self.program_to_application_send
            .send(SetMousePosition { x, y })
            .unwrap();
    }

    pub fn new_window(&mut self) -> crate::window_builder::WindowBuilder {
        crate::window_builder::WindowBuilder::new(self)
    }

    // Shouldn't be part of the public interface?
    /// If this instance of Application is on the main thread, process all application events.
    pub fn flush_events(&mut self) {
        if let Some(platform_application) = self.platform_application.as_mut() {
            platform_application.flush_events();
        }
    }
}

// When the application is dropped, quit the program.
impl Drop for Application {
    fn drop(&mut self) {
        self.quit();
    }
}
