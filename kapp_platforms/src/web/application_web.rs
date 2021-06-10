use kapp_platform_common::*;

pub struct PlatformApplication {}

impl PlatformApplicationTrait for PlatformApplication {
    type EventLoop = PlatformEventLoop;
    fn new() -> Self {
        Self {}
    }

    fn event_loop(&mut self) -> Self::EventLoop {
        PlatformEventLoop {}
    }

    fn set_window_position(&mut self, _window_id: WindowId, _x: u32, _y: u32) {}
    fn set_window_size(&mut self, _window_id: WindowId, _width: u32, _height: u32) {}
    fn set_window_title(&mut self, _window_id: WindowId, _title: &str) {}
    fn minimize_window(&mut self, _window_id: WindowId) {}
    fn maximize_window(&mut self, _window_id: WindowId) {}
    fn get_window_size(&mut self, _window_id: WindowId) -> (u32, u32) {
        todo!()
    }
    fn get_window_scale(&mut self, _window_id: WindowId) -> f64 {
        todo!()
    }
    fn fullscreen_window(&mut self, _window_id: WindowId) {
        todo!()
    }
    fn restore_window(&mut self, _window_id: WindowId) {
        todo!()
    }
    fn close_window(&mut self, _window_id: WindowId) {}
    fn redraw_window(&mut self, _window_id: WindowId) {
        todo!()
    }

    fn lock_mouse_position(&mut self) {
        todo!()
    }

    fn unlock_mouse_position(&mut self) {
        todo!()
    }

    fn new_window(&mut self, _window_parameters: &WindowParameters) -> WindowId {
        todo!()
    }

    fn quit(&self) {}

    fn set_cursor(&mut self, cursor: Cursor) {
        todo!()
    }
    fn hide_cursor(&mut self) {
        todo!()
    }
    fn show_cursor(&mut self) {
        todo!()
    }

    fn raw_window_handle(&self, _window_id: WindowId) -> RawWindowHandle {
        todo!()
    }

    fn start_text_input(&mut self) {}

    fn end_text_input(&mut self) {}

    fn set_text_input_rectangle(
        &mut self,
        _window_id: WindowId,
        _x: f64,
        _y: f64,
        _width: f64,
        _height: f64,
    ) {
        // Perhaps a hidden text input field could be moved to make IME input appear in the right place.
    }
}

// When the application is dropped, quit the program.
impl Drop for PlatformApplication {
    fn drop(&mut self) {
        self.quit();
    }
}

pub struct PlatformEventLoop {}

impl PlatformEventLoopTrait for PlatformEventLoop {
    fn run(&self, callback: Box<dyn FnMut(Event)>) {
        todo!()
    }
}
