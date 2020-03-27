#[link(name = "AppKit", kind = "framework")]
extern "C" {}
#[macro_use]
extern crate objc;

use kettlewin_platform_common::{
    Key, MouseButton, PlatformApplicationTrait, PlatformChannelTrait, PlatformWakerTrait,
    WindowParameters,
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

//pub use gl_context::GLContext;
use window_mac::WindowId;

pub type ApplicationMessage = kettlewin_platform_common::ApplicationMessage<crate::WindowId>;
pub type Event = kettlewin_platform_common::Event<crate::WindowId>;

pub mod prelude {
    pub use super::{
        application_mac::*,
        application_mac::{PlatformApplication, PlatformChannel, PlatformWaker},
        window_mac::WindowId,
        ApplicationMessage, Event,
    };

    pub mod events {
        pub use kettlewin_platform_common::Event::*;
    }

    pub use kettlewin_platform_common::{
        single_value_channel, Key, MouseButton, PlatformApplicationTrait, PlatformChannelTrait,
        PlatformWakerTrait, WindowParameters,
    };
}
