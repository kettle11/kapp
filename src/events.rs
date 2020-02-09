use crate::buttons::Button;

pub enum MouseButton {
    Left,
    Right,
    Middle,
    Unknown,
    __Nonexhaustive, // More buttons may be added.
}

pub enum Event {
    Draw, // The 'Draw' event can be seen as a recommendation of when to draw. It is not an actual system event.
    ButtonDown {
        button: Button,
        scancode: u32,
    },
    ButtonUp {
        button: Button,
        scancode: u32,
    },
    ButtonRepeat {
        button: Button,
    },
    MinimizedWindow,
    MaximizedWindow,
    ResizedWindow {
        width: u32,
        height: u32,
    },
    MouseMoved {
        x: f32,
        y: f32,
    },
    #[doc(hidden)]
    __Nonexhaustive, // More events will be added
}
