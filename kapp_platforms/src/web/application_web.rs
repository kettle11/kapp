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
        /*
        // This approach does not work for multiple canvases.
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let canvas_client_width = canvas.client_width() as u32;
        let canvas_client_height = canvas.client_height() as u32;
        (canvas_client_width, canvas_client_height)
        */
        unimplemented!()
    }
    fn get_window_scale(&mut self, _window_id: WindowId) -> f64 {
        unimplemented!()
    }
    fn fullscreen_window(&mut self, _window_id: WindowId) {
        // super::event_loop_web::request_fullscreen()
    }
    fn restore_window(&mut self, _window_id: WindowId) {
        unimplemented!()
    }
    fn close_window(&mut self, _window_id: WindowId) {}

    fn redraw_window(&mut self, _window_id: WindowId) {
        super::event_loop_web::request_animation_frame()
    }

    fn lock_mouse_position(&mut self) {
        unimplemented!()
    }

    fn unlock_mouse_position(&mut self) {
        unimplemented!();
    }

    fn new_window(&mut self, _window_parameters: &WindowParameters) -> WindowId {
        WindowId::new(0 as *mut std::ffi::c_void)
    }

    fn quit(&self) {}

    fn set_cursor(&mut self, _cursor: Cursor) {
        unimplemented!();
    }
    fn hide_cursor(&mut self) {
        unimplemented!()
    }
    fn show_cursor(&mut self) {
        unimplemented!()
    }

    fn raw_window_handle(&self, _window_id: WindowId) -> RawWindowHandle {
        RawWindowHandle::Web(raw_window_handle::web::WebHandle::empty())
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
        super::event_loop_web::run(callback);
    }
}
