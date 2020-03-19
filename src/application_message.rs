use crate::WindowId;
use std::sync::mpsc;

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
        window_parameters: crate::window_builder::WindowParameters,
        response_channel: mpsc::Sender<Result<WindowId, ()>>,
    },
}
