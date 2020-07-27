use kapp_platform_common::*;

pub struct PlatformEventLoop {}

impl PlatformEventLoopTrait for PlatformEventLoop {
    fn run(&self, callback: Box<dyn FnMut(Event)>) {
        unimplemented!()
    }
}

pub struct PlatformApplication {}

impl PlatformApplicationTrait for PlatformApplication {
    type EventLoop = PlatformEventLoop;

    fn new() -> Self {
        unimplemented!();
    }

    fn event_loop(&mut self) -> Self::EventLoop {
        unimplemented!();
    }

    fn set_window_position(&mut self, window_id: WindowId, x: u32, y: u32) {
        unimplemented!();
    }

    fn set_window_size(&mut self, window_id: WindowId, width: u32, height: u32) {
        unimplemented!();
    }

    fn set_window_title(&mut self, window_id: WindowId, title: &str) {
        unimplemented!();
    }

    fn minimize_window(&mut self, window_id: WindowId) {
        unimplemented!();
    }

    fn maximize_window(&mut self, _window_id: WindowId) {
        unimplemented!();
    }

    fn fullscreen_window(&mut self, window_id: WindowId) {
        unimplemented!();
    }

    fn restore_window(&mut self, _window_id: WindowId) {
        unimplemented!()
    }

    fn close_window(&mut self, window_id: WindowId) {
        unimplemented!();
    }

    fn redraw_window(&mut self, window_id: WindowId) {
        unimplemented!();
    }

    fn get_window_size(&mut self, window_id: WindowId) -> (u32, u32) {
        unimplemented!();
    }

    fn get_window_scale(&mut self, window_id: WindowId) -> f64 {
        unimplemented!();
    }

    fn set_mouse_position(&mut self, _x: u32, _y: u32) {
        unimplemented!();
    }

    // https://developer.apple.com/documentation/appkit/nscursor?language=objc
    fn set_cursor(&mut self, cursor: Cursor) {
        unimplemented!();
    }

    fn hide_cursor(&mut self) {
        unimplemented!();
    }

    fn show_cursor(&mut self) {
        unimplemented!();
    }

    fn new_window(&mut self, window_parameters: &WindowParameters) -> WindowId {
        unimplemented!();
    }

    fn quit(&self) {
        unimplemented!();
    }

    fn raw_window_handle(&self, window_id: WindowId) -> RawWindowHandle {
        unimplemented!();
    }
}

// When the application is dropped, quit the program.
impl Drop for PlatformApplication {
    fn drop(&mut self) {
        self.quit();
    }
}
