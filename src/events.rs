use crate::buttons::Button;
use crate::WindowId;
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Unknown,
    __Nonexhaustive, // More buttons may be added.
}

#[derive(Debug)]
pub enum Event {
    /// The 'Draw' event can be seen as a recommendation of when to draw. It is not an actual system event.
    Draw,
    // ------------------- Input Events  ---------------------
    // These are received by a window, but the window must be tracked with the 'GainedFocus' event.
    ButtonDown {
        button: Button,
    },
    ButtonUp {
        button: Button,
    },
    ButtonRepeat {
        button: Button,
    },
    MouseMoved {
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
    WindowClose {
        window_id: WindowId,
    },
    // ------------------- Application Events  ---------------------
    Quit,
    #[doc(hidden)]
    __Nonexhaustive, // More events will be added
}
