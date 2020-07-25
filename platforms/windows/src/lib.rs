mod application_windows;
mod event_loop_windows;
mod keys_windows;
mod utils_windows;

#[allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]
mod external_windows;

use kapp_platform_common::{
    raw_window_handle, Cursor, Event, Key, PointerSource, PointerButton, PlatformApplicationTrait,
    PlatformEventLoopTrait, RawWindowHandle, WindowId, WindowParameters,
};

pub mod prelude {
    pub use super::application_windows::{PlatformApplication, PlatformEventLoop};
    pub use kapp_platform_common::*;
}
