use super::keys_linux::*;
use kapp_platform_common::*;
use std::ffi::CString;
use std::ptr::{null, null_mut};
use std::time::Duration;

pub struct PlatformEventLoop {
    x_display: *mut c_void,
}

impl PlatformEventLoopTrait for PlatformEventLoop {
    fn run(&self, mut callback: Box<dyn FnMut(Event)>) {
        let mut event = XEvent {
            max_aligned_size: [0; 24],
        };
        // Used to peek ahead
        let mut next_event = XEvent {
            max_aligned_size: [0; 24],
        };

        let mut key_down = [false; 255];

        loop {
            unsafe {
                XNextEvent(self.x_display, &mut event);
                match event.type_ {
                    KeyPress => {
                        // If a key down occurs without a matching key key up
                        // then it's a repeat.
                        // This occurs because a flag is set during initialization with a call to:
                        // XkbSetDetectableAutoRepeat
                        // Are uppercase and lower case repeats properly accounted for?
                        // It seems that if a key down of 'a' occurs followed by a key up of 'A'
                        // that a bunch of key repeat events are received continuously. Which 
                        // feels like a bug, but not of this library. 
                        if key_down[event.key.keycode as usize] {
                            key_down[event.key.keycode as usize] = true;
                            callback(Event::KeyRepeat {
                                key: self.get_key(event.key.keycode),
                                timestamp: Duration::from_millis(event.key.time),
                            });
                        } else {
                            key_down[event.key.keycode as usize] = true;
                            callback(Event::KeyDown {
                                key: self.get_key(event.key.keycode),
                                timestamp: Duration::from_millis(event.key.time),
                            });
                        }
                    }
                    KeyRelease => {
                        key_down[event.key.keycode as usize] = false;
                        callback(Event::KeyUp {
                            key: self.get_key(event.key.keycode),
                            timestamp: Duration::from_millis(event.key.time),
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}

impl PlatformEventLoop {
    unsafe fn get_key(&self, key: c_uint) -> Key {
        let mut _length = 0;
        let key = *XGetKeyboardMapping(self.x_display, key as c_uchar, 1, &mut _length);
        virtual_keycode_to_key(key)
    }
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

            // Ensure that we can detect auto-repeated keys without funky code.
            let mut supported = false;
            XkbSetDetectableAutoRepeat(x_display, true, &mut supported);
            if !supported {
                // How likely is this?
                panic!("Detectable auto key repeat not supported");
            }

            PlatformApplication { x_display }
        }
    }

    fn event_loop(&mut self) -> Self::EventLoop {
        PlatformEventLoop {
            x_display: self.x_display,
        }
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
        // XDestroyWindow
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
        unsafe {
            let (width, height) = window_parameters.size.map_or((500, 500), |size| size);

            // Get the default screen
            let screen = XDefaultScreen(self.x_display);

            // Get colors to use for the window foreground and background.
            let black = XBlackPixel(self.x_display, screen);
            let white = XWhitePixel(self.x_display, screen);

            // In X11 all windows are children of another window.
            // The full-screen is considered a window, so our window will be a child of the
            // default full-screen 'root' window.
            let default_root_window = check_not_zero(XDefaultRootWindow(self.x_display)).unwrap();

            let window = check_not_zero(XCreateSimpleWindow(
                self.x_display,
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

            let window_name = CString::new(window_parameters.title.to_owned()).unwrap();
            let icon_name = CString::new("Hi").unwrap();

            // Note that these calls can return '1' but that is not an error.
            // However it's unclear what '1' means.
            XSetStandardProperties(
                self.x_display,
                window,
                window_name.as_ptr(),
                null_mut(), /* Icon name */
                0,          /* Pixel icon. 0 is 'None' */
                null_mut(), /* argv */
                0,          /* argc */
                null_mut(), /* Size hints */
            );

            XSelectInput(
                self.x_display,
                window,
                ExposureMask | ButtonPressMask | KeyPressMask | KeyReleaseMask,
            );

            let graphics_context = XCreateGC(
                self.x_display,
                window,
                0,          /* A value mask of the flags to set */
                null_mut(), /* A pointer to values to set as specified by the value mask */
            );

            // Clears the window to its backing color
            XClearWindow(self.x_display, window);

            // Make the window visible and raises it to the top
            XMapRaised(self.x_display, window);
            // XMapWindow(x_display, window);

            // X11 buffers commands until it reaches certain points, but we
            // can force it to flush the commands here and perform them immediately.
            XFlush(self.x_display);
            WindowId::new(window as *mut c_void)
        }
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

        // Is this the right spot for this?
        unsafe {
            XCloseDisplay(self.x_display);
        }
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
use std::os::raw::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong};

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
    fn XCloseDisplay(display: *mut c_void) -> c_int;
    fn XNextEvent(display: *mut c_void, event: *mut XEvent) -> c_int;
    //fn XPeekEvent(display: *mut c_void, event: *mut XEvent) -> c_int;
    //fn XEventsQueued(display: *mut c_void, mode: c_int) -> c_int;
    fn XGetKeyboardMapping(
        display: *mut c_void,
        first_keycode: c_uchar,
        keycode_count: c_int,
        keysyms_per_keycode_return: *mut c_int,
    ) -> *mut c_ulong;

    fn XkbSetDetectableAutoRepeat(
        display: *mut c_void,
        detectable: bool,
        supported_rtrn: *mut bool,
    ) -> bool;
}

pub type XID = c_ulong;
pub type Window = XID;

pub const KeyPressMask: c_long = 1 << 0;
pub const KeyReleaseMask: c_long = 2;

pub const ButtonPressMask: c_long = 1 << 2;
pub const ExposureMask: c_long = 1 << 15;

pub const KeyPress: c_int = 2;
pub const KeyRelease: c_int = 3;

pub const QueuedAfterReading: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub union XEvent {
    // Every XEvent variant implements 'type' as its first member so this variant is always valid.
    pub type_: c_int,
    pub key: XKeyEvent,
    /* Many XEvent union variant are excluded here */
    max_aligned_size: [u64; 24usize],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: bool,
    pub display: *mut c_void,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: c_ulong,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub keycode: c_uint,
    pub same_screen: bool,
}
