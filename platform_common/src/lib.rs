/// Structures and traits shared between the platform backends.
mod cursors;
mod events;
mod keys;
mod mouse_buttons;
mod platform_traits;
mod window_id;
mod window_parameters;

pub use cursors::Cursor;
pub use events::Event;
pub use keys::Key;
pub use mouse_buttons::MouseButton;
pub use platform_traits::{PlatformApplicationTrait, PlatformEventLoopTrait};
pub use window_id::WindowId;
pub use window_parameters::WindowParameters;
