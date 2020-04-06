use crate::{
    Cursor, Event, PlatformApplicationTrait, PlatformEventLoopTrait, WindowId, WindowParameters,
};
pub struct PlatformApplication {}

impl PlatformApplicationTrait for PlatformApplication {
    type EventLoop = PlatformEventLoop;
    fn new() -> Self {
        Self {}
    }

    fn event_loop(&mut self) -> Self::EventLoop {
        PlatformEventLoop {}
    }
    fn set_window_position(&mut self, window_id: WindowId, x: u32, y: u32) {}
    fn set_window_dimensions(&mut self, window_id: WindowId, width: u32, height: u32) {}
    fn set_window_title(&mut self, window_id: WindowId, title: &str) {}
    fn minimize_window(&mut self, window_id: WindowId) {}
    fn maximize_window(&mut self, window_id: WindowId) {}
    fn fullscreen_window(&mut self, window_id: WindowId) {
        super::event_loop_web::request_fullscreen()
    }
    fn restore_window(&mut self, window_id: WindowId) {
        unimplemented!()
    }
    fn close_window(&mut self, window_id: WindowId) {}
    fn redraw_window(&mut self, window_id: WindowId) {
        super::event_loop_web::request_frame()
    }

    fn set_mouse_position(&mut self, x: u32, y: u32) {
        unimplemented!()
    }

    fn new_window(&mut self, window_parameters: &WindowParameters) -> WindowId {
        WindowId::new(0 as *mut std::ffi::c_void)
    }

    fn quit(&mut self) {}
    fn set_cursor(&mut self, cursor: Cursor) {
        unimplemented!();
    }
    fn hide_cursor(&mut self) {
        unimplemented!()
    }
    fn show_cursor(&mut self) {
        unimplemented!()
    }
}

pub struct PlatformEventLoop {}

impl PlatformEventLoopTrait for PlatformEventLoop {
    fn run(&mut self, callback: Box<dyn FnMut(crate::Event)>) {
        super::event_loop_web::run(callback);
    }
}
