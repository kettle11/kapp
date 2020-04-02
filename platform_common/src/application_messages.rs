use crate::single_value_channel;
use crate::window_parameters::WindowParameters;
use crate::WindowId;

pub enum ApplicationMessage {
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
    SetWindowTitle {
        window: WindowId,
        title: String,
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
    RequestFrame {
        window: WindowId,
    },
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
