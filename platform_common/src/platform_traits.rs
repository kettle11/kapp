use crate::{Cursor, WindowId, WindowParameters};
pub trait PlatformApplicationTrait {
    type EventLoop: PlatformEventLoopTrait;

    fn new() -> Self;
    fn event_loop(&mut self) -> Self::EventLoop;
    fn set_window_position(&mut self, window_id: WindowId, x: u32, y: u32);
    fn set_window_dimensions(&mut self, window_id: WindowId, width: u32, height: u32);
    fn set_window_title(&mut self, window_id: WindowId, title: &str);
    fn minimize_window(&mut self, window_id: WindowId);
    fn maximize_window(&mut self, window_id: WindowId);
    fn fullscreen_window(&mut self, window_id: WindowId);
    fn restore_window(&mut self, window_id: WindowId);
    fn close_window(&mut self, window_id: WindowId);
    fn redraw_window(&mut self, window_id: WindowId);
    fn set_mouse_position(&mut self, x: u32, y: u32);
    fn new_window(&mut self, window_parameters: &WindowParameters) -> WindowId;
    fn quit(&mut self);
    fn set_cursor(&mut self, cursor: Cursor);
    fn hide_cursor(&mut self);
    fn show_cursor(&mut self);
}

pub trait PlatformEventLoopTrait {
    fn run(&mut self, callback: Box<dyn FnMut(crate::Event)>);
}
