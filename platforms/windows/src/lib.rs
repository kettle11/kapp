/// This is specifically a 'web' backend and not a general 'wasm' backend
/// In the future Wasm may be a more general purpose platform, and this backend
/// is not appropriate for all wasm contexts.
mod application_windows;
mod event_loop_windows;
mod keys_windows;
mod utils_windows;

use kettlewin_platform_common::{
    raw_window_handle, Cursor, Event, Key, MouseButton, PlatformApplicationTrait,
    PlatformEventLoopTrait, RawWindowHandle, WindowId, WindowParameters,
};

pub mod prelude {
    pub use super::application_windows::{PlatformApplication, PlatformEventLoop};
    pub use kettlewin_platform_common::*;
}
