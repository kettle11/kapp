pub mod prelude {
    pub use super::*;
    pub use kapp_platform_common::*;
}

mod keys_sdl;
use keys_sdl::*;

use fermium::{events::*, keyboard::*, mouse::*, rect::*, stdinc::*, touch::*, video::*, *};
use kapp_platform_common::*;
use std::ffi::{CStr, CString};
use std::time::Duration;
pub struct PlatformApplication {
    // These cursors are deallocated with `SDL_FreeCursor` in PlatformApplication's Drop
    arrow_cursor: *mut SDL_Cursor,
    ibeam_cursor: *mut SDL_Cursor,
    open_hand_cursor: *mut SDL_Cursor,
}

impl PlatformApplicationTrait for PlatformApplication {
    type EventLoop = PlatformEventLoop;
    fn new() -> Self {
        unsafe {
            assert!(SDL_Init(SDL_INIT_EVERYTHING) == 0);

            Self {
                arrow_cursor: SDL_CreateSystemCursor(SDL_SYSTEM_CURSOR_ARROW),
                ibeam_cursor: SDL_CreateSystemCursor(SDL_SYSTEM_CURSOR_IBEAM),
                open_hand_cursor: SDL_CreateSystemCursor(SDL_SYSTEM_CURSOR_HAND),
            }
        }
    }

    fn event_loop(&mut self) -> Self::EventLoop {
        PlatformEventLoop {}
    }

    fn set_window_position(&mut self, window_id: WindowId, x: u32, y: u32) {
        unsafe {
            SDL_SetWindowPosition(window_id.raw() as *mut SDL_Window, x as i32, y as i32);
        }
    }
    fn set_window_size(&mut self, window_id: WindowId, width: u32, height: u32) {
        unsafe {
            SDL_SetWindowPosition(
                window_id.raw() as *mut SDL_Window,
                width as i32,
                height as i32,
            );
        }
    }
    fn set_window_title(&mut self, window_id: WindowId, title: &str) {
        unsafe {
            let c_string = CString::new(title).unwrap();
            SDL_SetWindowTitle(window_id.raw() as *mut SDL_Window, c_string.as_ptr());
        }
    }
    fn minimize_window(&mut self, window_id: WindowId) {
        unsafe {
            SDL_MinimizeWindow(window_id.raw() as *mut SDL_Window);
        }
    }
    fn maximize_window(&mut self, window_id: WindowId) {
        unsafe {
            SDL_MaximizeWindow(window_id.raw() as *mut SDL_Window);
        }
    }
    fn get_window_size(&mut self, window_id: WindowId) -> (u32, u32) {
        let mut width = 0;
        let mut height = 0;
        unsafe {
            SDL_GetWindowSize(window_id.raw() as *mut SDL_Window, &mut width, &mut height);
        }
        (width as u32, height as u32)
    }

    fn get_window_scale(&mut self, window_id: WindowId) -> f64 {
        let (logical_width, _logical_height) = self.get_window_size(window_id);
        let mut physical_width = 0;
        let mut _physical_height = 0;

        // This call returns the actual pixel widths that would be in a framebuffer.
        unsafe {
            SDL_GL_GetDrawableSize(
                window_id.raw() as *mut SDL_Window,
                &mut physical_width,
                &mut _physical_height,
            );
        }
        logical_width as f64 / physical_width as f64
    }

    fn fullscreen_window(&mut self, window_id: WindowId) {
        unsafe {
            SDL_SetWindowFullscreen(window_id.raw() as *mut SDL_Window, SDL_WINDOW_FULLSCREEN.0);
        }
    }
    fn restore_window(&mut self, window_id: WindowId) {
        unsafe {
            SDL_RestoreWindow(window_id.raw() as *mut SDL_Window);
        }
    }
    fn close_window(&mut self, window_id: WindowId) {
        unsafe {
            SDL_DestroyWindow(window_id.raw() as *mut SDL_Window);
        }
    }
    fn redraw_window(&mut self, _window_id: WindowId) {
        // Does nothing on the SDL backend.
    }

    fn lock_mouse_position(&mut self) {
        unsafe {
            SDL_SetRelativeMouseMode(SDL_TRUE);
        }
    }

    fn unlock_mouse_position(&mut self) {
        unsafe {
            SDL_SetRelativeMouseMode(SDL_FALSE);
        }
    }

    fn new_window(&mut self, window_parameters: &WindowParameters) -> WindowId {
        let (x, y) = window_parameters.position.unwrap_or((
            SDL_WINDOWPOS_UNDEFINED as u32,
            SDL_WINDOWPOS_UNDEFINED as u32,
        ));

        // TODO: Width and height are presently incorrect as SDL interprets them as logical pixels.
        // DPI scale factor needs to be accounted for.
        let (width, height) = window_parameters.size.unwrap();

        // SDL_WINDOW_OPENGL is probably not something `kapp`
        // wants to assume.
        // But this is tolerable for now.
        let mut flags = SDL_WINDOW_OPENGL | SDL_WINDOW_ALLOW_HIGHDPI;
        if window_parameters.resizable {
            flags |= SDL_WINDOW_RESIZABLE;
        }

        if window_parameters.resizable {
            flags |= SDL_WINDOW_RESIZABLE;
        }
        unsafe {
            let window = SDL_CreateWindow(
                b"demo\0".as_ptr().cast(),
                x as i32,
                y as i32,
                (width / 2) as i32,
                (height / 2) as i32,
                flags.0,
            );

            // How can min / max sizes be unset later?
            if let Some((min_width, min_height)) = window_parameters.minimum_size {
                SDL_SetWindowMinimumSize(window, min_width as i32, min_height as i32)
            }

            if let Some((max_width, max_height)) = window_parameters.maximum_size {
                SDL_SetWindowMaximumSize(window, max_width as i32, max_height as i32)
            }

            let c_string = std::ffi::CString::new(window_parameters.title.clone()).unwrap();
            SDL_SetWindowTitle(window, c_string.as_ptr());

            WindowId::new(window as *mut c_void)
        }
    }

    fn quit(&self) {
        unsafe {
            SDL_Quit();
            // TODO: Instead of panicking the closure should be made no-longer reentrant.
            // Without this closure quitting infinitely loops
            panic!();
        }
    }

    fn set_cursor(&mut self, cursor: Cursor) {
        let cursor = match cursor {
            Cursor::IBeam => self.ibeam_cursor,
            Cursor::OpenHand => self.open_hand_cursor,
            _ => self.arrow_cursor,
        };
        unsafe {
            SDL_SetCursor(cursor);
        }
    }

    fn hide_cursor(&mut self) {
        unsafe {
            SDL_ShowCursor(SDL_DISABLE);
        }
    }
    fn show_cursor(&mut self) {
        unsafe {
            SDL_ShowCursor(SDL_ENABLE);
        }
    }

    fn raw_window_handle(&self, window_id: WindowId) -> RawWindowHandle {
        // TODO: This requires calling SDL_GetWindowWMInfo and placing the data
        // correctly into RawWindowHandle.
        unimplemented!()
    }

    fn start_text_input(&mut self) {
        unsafe {
            SDL_StartTextInput();
        }
    }

    fn end_text_input(&mut self) {
        unsafe {
            SDL_StopTextInput();
        }
    }

    fn set_text_input_rectangle(
        &mut self,
        _window_id: WindowId,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        let mut rectangle = SDL_Rect {
            x: x as c_int,
            y: y as c_int,
            w: width as c_int,
            h: height as c_int,
        };
        unsafe {
            SDL_SetTextInputRect(&mut rectangle);
        }
    }
}

// When the application is dropped, quit the program.
impl Drop for PlatformApplication {
    fn drop(&mut self) {
        unsafe {
            SDL_FreeCursor(self.arrow_cursor);
            SDL_FreeCursor(self.ibeam_cursor);
            SDL_FreeCursor(self.open_hand_cursor);
            SDL_Quit();
        }
    }
}

pub struct PlatformEventLoop {}

impl PlatformEventLoopTrait for PlatformEventLoop {
    fn run(&self, mut callback: Box<dyn FnMut(Event)>) {
        unsafe {
            let mut event = std::mem::zeroed();
            loop {
                SDL_WaitEvent(&mut event);

                match event.type_ {
                    SDL_QUIT => callback(Event::QuitRequested),
                    SDL_WINDOWEVENT => {
                        let window_event = event.window;
                        // TODO: Fermium doesn't expose SDL_GetWindowFromID
                        // let window_id = WindowId::new(SDL_GetWindowFromID(window_event.windowID));
                        let window_id = WindowId::new(std::ptr::null_mut());
                        match window_event.event {
                            SDL_WINDOWEVENT_CLOSE => {
                                callback(Event::WindowCloseRequested { window_id })
                            }
                            _ => {}
                        }
                    }
                    SDL_KEYDOWN | SDL_KEYUP => {
                        let keyboard_event = event.key;

                        // Are milliseconds the correct units?
                        let timestamp = Duration::from_millis(keyboard_event.timestamp as u64);

                        let key = scancode_to_key(keyboard_event.keysym.scancode);
                        match keyboard_event.type_ {
                            SDL_KEYDOWN => {
                                if keyboard_event.repeat > 0 {
                                    callback(Event::KeyRepeat { key, timestamp })
                                } else {
                                    callback(Event::KeyDown { key, timestamp })
                                }
                            }
                            SDL_KEYUP => callback(Event::KeyUp { key, timestamp }),
                            _ => {}
                        }
                    }
                    SDL_MOUSEMOTION => {
                        let mouse_motion_event = event.motion;

                        // Are milliseconds the correct units?
                        let timestamp = Duration::from_millis(mouse_motion_event.timestamp as u64);
                        let source = match mouse_motion_event.which {
                            SDL_TOUCH_MOUSEID => PointerSource::Touch,
                            _ => PointerSource::Mouse,
                        };

                        // Do these need to be scaled by the window DPI?
                        callback(Event::MouseMotion {
                            delta_x: mouse_motion_event.xrel as f64,
                            delta_y: mouse_motion_event.yrel as f64,
                            timestamp,
                        });
                        callback(Event::PointerMoved {
                            x: mouse_motion_event.x as f64,
                            y: mouse_motion_event.y as f64,
                            source,
                            timestamp,
                        });
                    }
                    SDL_MOUSEBUTTONDOWN => {
                        let event = event.button;

                        let source = match event.which {
                            SDL_TOUCH_MOUSEID => PointerSource::Touch,
                            _ => PointerSource::Mouse,
                        };

                        // Are milliseconds the correct units?
                        let timestamp = Duration::from_millis(event.timestamp as u64);
                        let button = match event.button as u32 {
                            SDL_BUTTON_LEFT => PointerButton::Primary,
                            SDL_BUTTON_MIDDLE => PointerButton::Auxillary,
                            SDL_BUTTON_RIGHT => PointerButton::Secondary,
                            SDL_BUTTON_X1 => PointerButton::Extra1,
                            SDL_BUTTON_X2 => PointerButton::Extra2,
                            _ => PointerButton::Unknown,
                        };

                        callback(Event::PointerDown {
                            x: event.x as f64,
                            y: event.y as f64,
                            source,
                            button,
                            timestamp,
                        });

                        if event.clicks == 2 {
                            callback(Event::DoubleClickDown {
                                x: event.x as f64,
                                y: event.y as f64,
                                button,
                                timestamp,
                            });
                            callback(Event::DoubleClick {
                                x: event.x as f64,
                                y: event.y as f64,
                                button,
                                timestamp,
                            });
                        }
                    }
                    SDL_MOUSEBUTTONUP => {
                        let event = event.button;

                        let source = match event.which {
                            SDL_TOUCH_MOUSEID => PointerSource::Touch,
                            _ => PointerSource::Mouse,
                        };

                        // Are milliseconds the correct units?
                        let timestamp = Duration::from_millis(event.timestamp as u64);
                        let button = match event.button as u32 {
                            SDL_BUTTON_LEFT => PointerButton::Primary,
                            SDL_BUTTON_MIDDLE => PointerButton::Auxillary,
                            SDL_BUTTON_RIGHT => PointerButton::Secondary,
                            SDL_BUTTON_X1 => PointerButton::Extra1,
                            SDL_BUTTON_X2 => PointerButton::Extra2,
                            _ => PointerButton::Unknown,
                        };
                        callback(Event::PointerUp {
                            x: event.x as f64,
                            y: event.y as f64,
                            source,
                            button,
                            timestamp,
                        });
                        if event.clicks == 2 {
                            callback(Event::DoubleClickUp {
                                x: event.x as f64,
                                y: event.y as f64,
                                button,
                                timestamp,
                            });
                        }
                    }
                    SDL_TEXTINPUT => {
                        let c_str = CStr::from_ptr(event.text.text.as_ptr()).to_str().unwrap();
                        for character in c_str.chars() {
                            // Send a character received for each key.
                            callback(Event::CharacterReceived { character });
                        }
                    }
                    SDL_TEXTEDITING => {
                        let text_event = event.text;
                        let c_str = CStr::from_ptr(event.text.text.as_ptr()).to_str().unwrap();
                        callback(Event::IMEComposition {
                            composition: c_str.to_string(),
                        });
                    }
                    _ => continue,
                }

                // If there are no events remaining, we're at the end of the event loop
                if SDL_HasEvents(SDL_FIRSTEVENT, SDL_LASTEVENT) == SDL_FALSE {
                    callback(Event::EventsCleared);

                    // TODO: Keep track of all windows and send a draw call to each here.
                    let window_id = WindowId::new(std::ptr::null_mut());
                    callback(Event::Draw { window_id });
                }
            }
        }
    }
}
