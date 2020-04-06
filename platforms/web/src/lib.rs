/// This is specifically a 'web' backend and not a general 'wasm' backend
/// In the future Wasm may be a more general purpose platform, and this backend
/// is not appropriate for all wasm contexts.
mod application_web;
mod event_loop_web;
mod keys_web;

use kettlewin_platform_common::{
    Cursor, Event, Key, MouseButton, PlatformApplicationTrait, PlatformEventLoopTrait, WindowId,
    WindowParameters,
};

pub mod prelude {
    pub use super::application_web::{PlatformApplication, PlatformEventLoop};
    pub use kettlewin_platform_common::{
        Cursor, Event, Key, MouseButton, PlatformApplicationTrait, PlatformEventLoopTrait,
        WindowId, WindowParameters,
    };
}
