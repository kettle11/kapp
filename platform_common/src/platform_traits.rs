/// These are the core functions to be implemented by each platform.
use crate::{Cursor, WindowId, WindowParameters, raw_window_handle::RawWindowHandle};
pub trait PlatformApplicationTrait {
    type EventLoop: PlatformEventLoopTrait;

    fn new() -> Self;
    fn event_loop(&mut self) -> Self::EventLoop;

    /// Sets window position in physical coordinates on its current screen.
    fn set_window_position(&mut self, window_id: WindowId, x: u32, y: u32);
    /// Sets window dimensions with physical coordinates.
    fn set_window_dimensions(&mut self, window_id: WindowId, width: u32, height: u32);
    fn set_window_title(&mut self, window_id: WindowId, title: &str);
    fn minimize_window(&mut self, window_id: WindowId);
    fn maximize_window(&mut self, window_id: WindowId);
    fn fullscreen_window(&mut self, window_id: WindowId);
    /// Returns the window to the state where it's not minimized, maximized, or fullscreen
    fn restore_window(&mut self, window_id: WindowId);
    fn close_window(&mut self, window_id: WindowId);

    /// Requests that the a Draw event be sent for the window.
    /// Draw events should either be sent at the end of an event loop,
    /// or in response to a system redraw request.
    /// If multiple window redraws are requested no ordering should be assumed.
    fn redraw_window(&mut self, window_id: WindowId);

    /// Sets the mouse position in physical coordinates in relation to the screen.
    fn set_mouse_position(&mut self, x: u32, y: u32);
    fn new_window(&mut self, window_parameters: &WindowParameters) -> WindowId;
    fn quit(&self);

    /// Sets the cursor in a way that persists between all windows for the current program.
    fn set_cursor(&mut self, cursor: Cursor);

    /// Hides the cursor or this application until a call to show cursor.
    fn hide_cursor(&mut self);
    fn show_cursor(&mut self);
    fn raw_window_handle(&self, window_id: WindowId) -> RawWindowHandle;
}

pub trait PlatformEventLoopTrait {
    /// Runs until the application quits.
    fn run(& self, callback: Box<dyn FnMut(crate::Event)>);
}
