use crate::single_value_channel;
use crate::window_parameters::WindowParameters;

pub enum ApplicationMessage<WindowId> {
    SetWindowPosition {
        window: WindowId,
        x: u32,
        y: u32,
    },
    SetWindowSize {
        window: WindowId,
        width: u32,
        height: u32,
    },
    MinimizeWindow {
        window: WindowId,
    },
    MaximizeWindow {
        window: WindowId,
    },
    FullscreenWindow {
        window: WindowId,
    },
    RestoreWindow {
        window: WindowId,
    },
    DropWindow {
        window: WindowId,
    },
    RequestFrame,
    Quit,
    SetMousePosition {
        x: u32,
        y: u32,
    },
    NewWindow {
        window_parameters: WindowParameters,
        response_channel: single_value_channel::Sender<Result<WindowId, ()>>,
    },
}
