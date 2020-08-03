use kapp_platform_common::*;
use std::ffi::CString;
use std::ptr::{null, null_mut};

pub struct PlatformEventLoop {}

impl PlatformEventLoopTrait for PlatformEventLoop {
    fn run(&self, callback: Box<dyn FnMut(Event)>) {}
}

pub struct PlatformApplication {
    x_display: *mut c_void,
}

impl PlatformApplicationTrait for PlatformApplication {
    type EventLoop = PlatformEventLoop;

    fn new() -> Self {
        unsafe {
            // Open a connection to the display server
            let x_display = check_not_null(XOpenDisplay(null())).unwrap();

            // Get the default screen
            let screen = XDefaultScreen(x_display);

            // Get colors to use for the window foreground and background.
            let black = XBlackPixel(x_display, screen);
            let white = XWhitePixel(x_display, screen);

            // In X11 all windows are children of another window.
            // The full-screen is considered a window, so our window will be a child of the
            // default full-screen 'root' window.
            let default_root_window = check_not_zero(XDefaultRootWindow(x_display)).unwrap();

            let window = check_not_zero(XCreateSimpleWindow(
                x_display,
                default_root_window,
                10,    /* x */
                10,    /* y */
                300,   /* width */
                300,   /* height */
                1,     /* border width */
                white, /* border color */
                black, /* background color */
            ))
            .unwrap();

            let window_name = CString::new("My Window").unwrap();
            let icon_name = CString::new("Hi").unwrap();

            // Note that these calls can return '1' but that is not an error.
            // However it's unclear what '1' means.
            XSetStandardProperties(
                x_display,
                window,
                window_name.as_ptr(),
                null_mut(), /* Icon name */
                0,          /* Pixel icon. 0 is 'None' */
                null_mut(), /* argv */
                0,          /* argc */
                null_mut(), /* Size hints */
            );

            XSelectInput(
                x_display,
                window,
                ExposureMask | ButtonPressMask | KeyPressMask,
            );

            let graphics_context = XCreateGC(
                x_display,
                window,
                0,          /* A value mask of the flags to set */
                null_mut(), /* A pointer to values to set as specified by the value mask */
            );

            XClearWindow(x_display, window);
            XMapRaised(x_display, window);
            // XMapWindow(x_display, window);

            XFlush(x_display);

            PlatformApplication { x_display }
        }
    }

    fn event_loop(&mut self) -> Self::EventLoop {
        PlatformEventLoop {}
    }

    fn set_window_position(&mut self, window_id: WindowId, x: u32, y: u32) {
        unimplemented!();
    }

    fn set_window_size(&mut self, window_id: WindowId, width: u32, height: u32) {
        unimplemented!();
    }

    fn set_window_title(&mut self, window_id: WindowId, title: &str) {
        unimplemented!();
    }

    fn minimize_window(&mut self, window_id: WindowId) {
        unimplemented!();
    }

    fn maximize_window(&mut self, _window_id: WindowId) {
        unimplemented!();
    }

    fn fullscreen_window(&mut self, window_id: WindowId) {
        unimplemented!();
    }

    fn restore_window(&mut self, _window_id: WindowId) {
        unimplemented!()
    }

    fn close_window(&mut self, window_id: WindowId) {
        unimplemented!();
    }

    fn redraw_window(&mut self, window_id: WindowId) {
        unimplemented!();
    }

    fn get_window_size(&mut self, window_id: WindowId) -> (u32, u32) {
        unimplemented!();
    }

    fn get_window_scale(&mut self, window_id: WindowId) -> f64 {
        unimplemented!();
    }

    fn set_mouse_position(&mut self, _x: u32, _y: u32) {
        unimplemented!();
    }

    fn set_cursor(&mut self, cursor: Cursor) {
        unimplemented!();
    }

    fn hide_cursor(&mut self) {
        unimplemented!();
    }

    fn show_cursor(&mut self) {
        unimplemented!();
    }

    fn new_window(&mut self, window_parameters: &WindowParameters) -> WindowId {
        unimplemented!();
    }

    fn quit(&self) {
        unimplemented!();
    }

    fn raw_window_handle(&self, window_id: WindowId) -> RawWindowHandle {
        unimplemented!();
    }
}

// When the application is dropped, quit the program.
impl Drop for PlatformApplication {
    fn drop(&mut self) {
        self.quit();
    }
}

fn check_error(e: c_int) -> Result<(), c_int> {
    if e != 0 {
        Err(e)
    } else {
        Ok(())
    }
}

fn check_not_null(p: *mut c_void) -> Result<*mut c_void, ()> {
    if p == null_mut() {
        Err(())
    } else {
        Ok(p)
    }
}

fn check_not_zero(p: c_ulong) -> Result<c_ulong, ()> {
    if p == 0 {
        Err(())
    } else {
        Ok(p)
    }
}

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong};

#[link(name = "X11")]
extern "C" {
    pub fn XOpenDisplay(display_name: *const c_char) -> *mut c_void;
    pub fn XCreateSimpleWindow(
        display: *mut c_void,
        parent: XID,
        x: c_int,
        y: c_int,
        width: c_uint,
        height: c_uint,
        border_width: c_uint,
        border: c_ulong,
        background: c_ulong,
    ) -> Window;
    pub fn XDefaultScreen(display: *mut c_void) -> c_int;
    pub fn XBlackPixel(display: *mut c_void, _1: c_int) -> c_ulong;
    pub fn XWhitePixel(display: *mut c_void, _1: c_int) -> c_ulong;
    pub fn XDefaultRootWindow(_1: *mut c_void) -> c_ulong;
    fn XClearWindow(display: *mut c_void, window: Window) -> c_int;
    fn XMapRaised(display: *mut c_void, window: Window) -> c_int;
    fn XMapWindow(display: *mut c_void, window: Window) -> c_int;

    fn XCreateGC(
        display: *mut c_void,
        window: Window,
        value_mask: c_ulong,
        x_gc_values: *mut c_void,
    ) -> *mut c_void;

    fn XSetStandardProperties(
        display: *mut c_void,
        window: Window,
        window_name: *const c_char,
        icon_name: *const c_char,
        icon_pixmap: c_ulong,
        argv: *mut *mut c_char,
        argc: c_int,
        size_hints: *mut c_void,
    ) -> c_int;
    fn XSelectInput(display: *mut c_void, window: Window, event_mask: c_long) -> c_int;
    fn XFlush(display: *mut c_void);
}

pub type XID = c_ulong;
pub type Window = XID;

pub const KeyPressMask: c_long = 1 << 0;
pub const ButtonPressMask: c_long = 1 << 2;
pub const ExposureMask: c_long = 1 << 15;
