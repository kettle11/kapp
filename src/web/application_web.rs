use std::io::Error;

pub struct Window {
    pub id: WindowId,
}

#[derive(Debug, Eq, PartialEq)]
pub struct WindowId {}

pub struct WindowBuilder<'a> {
    _application: &'a Application,
}

impl<'a> WindowBuilder<'a> {
    pub fn build(&self) -> Result<Window, Error> {
        Ok(Window { id: WindowId {} })
    }
    pub fn title(&mut self, _title: &'a str) -> &mut Self {
        self
    }
    pub fn position(&mut self, _x: u32, _y: u32) -> &mut Self {
        self
    }
    pub fn dimensions(&mut self, _width: u32, _height: u32) -> &mut Self {
        self
    }
}

pub struct ApplicationBuilder {}

impl ApplicationBuilder {
    pub fn build(&self) -> Result<Application, Error> {
        Ok(Application {
            window_constructed: false,
        })
    }
}

// Clone should not be public
#[derive(Clone)]

pub struct Application {
    window_constructed: bool,
}

impl Application {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder {}
    }

    pub fn new_window<'a>(&'a mut self) -> WindowBuilder<'a> {
        if self.window_constructed {
            // Only one 'window' matters on web, should some sort of warning be issued here?
        }
        self.window_constructed = true;
        WindowBuilder { _application: self }
    }

    pub fn quit(&self) {}

    pub fn event_loop(&mut self) -> EventLoop {
        EventLoop {}
    }

    pub fn request_frame(&mut self) {
        super::event_loop_web::request_frame();
    }
}

pub struct EventLoop {}

impl EventLoop {
    pub fn run<T>(&self, callback: T)
    where
        T: 'static + FnMut(crate::Event),
    {
        super::event_loop_web::run(callback);
    }
}

impl Window {
    /// Returns the window from a fullscreen
    pub fn restore(&self) {
        unimplemented!()
    }

    pub fn fullscreen(&self) {
        unimplemented!()
    }
}
