use crate::keys::Key;
use crate::mouse_buttons::MouseButton;
use crate::WindowId;

/// Input and system events
#[derive(Debug, Clone)]
pub enum Event {
    /// The 'Draw' event can be seen as a recommendation for when to draw. It is not an actual system event.
    Draw {
        window_id: WindowId,
    },
    // ------------------- Input Events  ---------------------
    // These are received by a window, but the window must be tracked with the 'GainedFocus' event.
    /// A key is pressed.
    KeyDown {
        key: Key,
    },
    /// A key is released.
    KeyUp {
        key: Key,
    },
    /// A key is held and begins to repeat.
    KeyRepeat {
        key: Key,
    },
    /// The mouse position has changed. Reports physical coordinates.
    MouseMoved {
        x: f32,
        y: f32,
    },
    MouseButtonDown {
        x: f32,
        y: f32,
        button: MouseButton,
    },
    MouseButtonUp {
        x: f32,
        y: f32,
        button: MouseButton,
    },
    ScrollWheel {
        delta: f32,
    },
    /// Only available on MacOS. Reports absolute position on trackpad.
    TrackpadTouch {
        x: f32,
        y: f32,
    },
    // ------------------- Window Events  ---------------------
    WindowMinimized {
        window_id: WindowId,
    },
    /// This event will not be sent on MacOS, only the WindowFullscreened event will be sent.
    WindowMaximized {
        window_id: WindowId,
    },
    WindowFullscreened {
        window_id: WindowId,
    },
    /// A window is 'restored' when it returns from being minimized or maximized.
    WindowRestored {
        window_id: WindowId,
    },
    WindowStartResize {
        window_id: WindowId,
    },
    WindowEndResize {
        window_id: WindowId,
    },
    WindowResized {
        width: u32,
        height: u32,
        window_id: WindowId,
    },
    /// Reports the new x and y position for the *lower left* corner of the window.
    WindowMoved {
        x: u32,
        y: u32,
        window_id: WindowId,
    },
    WindowGainedFocus {
        window_id: WindowId,
    },
    WindowLostFocus {
        window_id: WindowId,
    },
    /// A close is requested for the WindowId.
    /// The Window will not actually close until the associated 'Window' is dropped.
    WindowCloseRequested {
        window_id: WindowId,
    },
    // ------------------- Application Events  ---------------------
    Quit,
    #[doc(hidden)]
    __Nonexhaustive, // More events will be added
}