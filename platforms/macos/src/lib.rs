#[link(name = "AppKit", kind = "framework")]
extern "C" {}
#[macro_use]
extern crate objc;

use kettlewin_platform_common::{
    ApplicationMessage, Event, Key, MouseButton, PlatformApplicationTrait, PlatformChannelTrait,
    PlatformWakerTrait, WindowId, WindowParameters,
};

#[allow(
    non_upper_case_globals,
    non_snake_case,
    dead_code,
    non_camel_case_types
)]
mod apple;
mod application_mac;
mod events_mac;
//mod gl_context;
mod keys_mac;
mod window_mac;

pub mod prelude {
    pub use super::{
        application_mac::*,
        application_mac::{PlatformApplication, PlatformChannel, PlatformWaker},
    };
    pub use kettlewin_platform_common::{
        single_value_channel, ApplicationMessage, Event, Key, MouseButton,
        PlatformApplicationTrait, PlatformChannelTrait, PlatformWakerTrait, WindowId,
        WindowParameters,
    };
}
