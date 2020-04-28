use crate::keys::Key;
use crate::mouse_buttons::MouseButton;
use crate::WindowId;
use std::time::Duration;
/// Input and system events
/// All user input events have timestamps.
/// Timestamps on MacOS and Windows represent time since the computer was turned on.
/// On Web timestamps represent time since the current document was created.
/// Precision of timestamps varies between platforms.
#[derive(Debug, Clone)]
pub enum Event {
    /// A recommendation for when to draw.
    /// On MacOS 'Draw' is sent after 'EventsCleared' or in response to
    /// a system event during resizing. While resizing draw will be sent at the screen's
    /// refresh rate.
    /// On Web Draw is triggered by requestAnimationFrame
    /// On Windows Draw is sent at the end of the event loop.
    Draw {
        window_id: WindowId,
    },
    // ------------------- Input Events ---------------------
    /// A key is pressed.
    KeyDown {
        key: Key,
        timestamp: Duration,
    },
    /// A key is released.
    KeyUp {
        key: Key,
        timestamp: Duration,
    },
    /// A repeat of a held key.
    KeyRepeat {
        key: Key,
        timestamp: Duration,
    },
    /// The mouse position has changed. Reports physical coordinates.
    MouseMoved {
        x: f32,
        y: f32,
        timestamp: Duration,
    },
    MouseButtonDown {
        x: f32,
        y: f32,
        button: MouseButton,
        timestamp: Duration,
    },
    MouseButtonUp {
        x: f32,
        y: f32,
        button: MouseButton,
        timestamp: Duration,
    },
    /// If delta_x is set it horizontal scrolling from something like a trackpad.
    /// Momentum may be added to this value
    Scroll {
        delta_x: f32,
        delta_y: f32,
        timestamp: Duration,
    },
    PinchGesture {
        delta: f32,
        timestamp: Duration,
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
    /// A window is 'restored' when it returns from being minimized, maximized, or fullscreened.
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
    /// Reports the new x and y position for the upper left corner of the window.
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
    /// The application is going to quit immediately after this event is processed.
    /// Perform any final cleanup that's necessary. The quit cannot be cancelled.
    Quit,
    /// A quit is requested, but it is up to the program to call quit().
    QuitRequested,
    /// When the event loop sends its last event
    EventsCleared,
    #[doc(hidden)]
    __Nonexhaustive, // More events will be added
}
