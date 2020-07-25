#[macro_use]
extern crate objc;

#[allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]
mod apple;
mod application_mac;
mod events_mac;
mod keys_mac;
mod window_mac;

use kapp_platform_common::{
    Cursor, Event, Key, PlatformApplicationTrait, PlatformEventLoopTrait, PointerButton,
    PointerSource, WindowId, WindowParameters,
};

pub mod prelude {
    pub use super::{application_mac::PlatformApplication, application_mac::*};
    pub use kapp_platform_common::*;
}
