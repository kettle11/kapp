/// This is specifically a 'web' backend and not a general 'wasm' backend
/// In the future Wasm may be a more general purpose platform, and this backend
/// is not appropriate for all wasm contexts.
mod application_web;
mod event_loop_web;
mod keys_web;

use kettlewin_platform_common::{
    ApplicationMessage, Event, Key, MouseButton, PlatformApplicationTrait, PlatformChannelTrait,
    PlatformWakerTrait, WindowId,
};

pub mod prelude {
    pub use super::{
        application_web::*,
        application_web::{PlatformApplication, PlatformChannel, PlatformWaker},
    };
    pub use kettlewin_platform_common::{
        single_value_channel, ApplicationMessage, Event, Key, MouseButton,
        PlatformApplicationTrait, PlatformChannelTrait, PlatformWakerTrait, WindowId,
        WindowParameters,
    };
}
