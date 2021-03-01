pub mod prelude {
    pub use super::*;
    pub use kapp_platform_common::*;
}

use fermium::{events::*, video::*, *};
use kapp_platform_common::*;

pub struct PlatformApplication {}

impl PlatformApplicationTrait for PlatformApplication {
    type EventLoop = PlatformEventLoop;
    fn new() -> Self {
        unsafe {
            assert!(SDL_Init(SDL_INIT_EVERYTHING) == 0);
            Self {}
        }
    }

    fn event_loop(&mut self) -> Self::EventLoop {
        PlatformEventLoop {}
    }

    fn set_window_position(&mut self, _window_id: WindowId, _x: u32, _y: u32) {}
    fn set_window_size(&mut self, _window_id: WindowId, _width: u32, _height: u32) {}
    fn set_window_title(&mut self, _window_id: WindowId, _title: &str) {}
    fn minimize_window(&mut self, _window_id: WindowId) {}
    fn maximize_window(&mut self, _window_id: WindowId) {}
    fn get_window_size(&mut self, _window_id: WindowId) -> (u32, u32) {
        unimplemented!()
    }
    fn get_window_scale(&mut self, _window_id: WindowId) -> f64 {
        unimplemented!()
    }
    fn fullscreen_window(&mut self, _window_id: WindowId) {}
    fn restore_window(&mut self, _window_id: WindowId) {
        unimplemented!()
    }
    fn close_window(&mut self, _window_id: WindowId) {}
    fn redraw_window(&mut self, _window_id: WindowId) {}

    fn lock_mouse_position(&mut self) {}

    fn unlock_mouse_position(&mut self) {}

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
        let mut flags = (SDL_WINDOW_OPENGL | SDL_WINDOW_ALLOW_HIGHDPI);
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
                width as i32,
                height as i32,
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

    fn set_cursor(&mut self, cursor: Cursor) {}
    fn hide_cursor(&mut self) {}
    fn show_cursor(&mut self) {}

    fn raw_window_handle(&self, _window_id: WindowId) -> RawWindowHandle {
        unimplemented!()
    }

    fn start_text_input(&mut self) {}

    fn end_text_input(&mut self) {}

    fn set_text_input_rectangle(
        &mut self,
        _window_id: WindowId,
        _x: f64,
        _y: f64,
        _width: f64,
        _height: f64,
    ) {
        // Perhaps a hidden text input field could be moved to make IME input appear in the right place.
    }
}

// When the application is dropped, quit the program.
impl Drop for PlatformApplication {
    fn drop(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }
}

pub struct PlatformEventLoop {}

impl PlatformEventLoopTrait for PlatformEventLoop {
    fn run(&self, mut callback: Box<dyn FnMut(Event)>) {
        unsafe {
            let mut sdl_event = std::mem::zeroed();
            loop {
                SDL_WaitEvent(&mut sdl_event);

                match sdl_event.type_ {
                    SDL_QUIT => callback(Event::QuitRequested),
                    SDL_WINDOWEVENT => {
                        let window_event = sdl_event.window;
                        // Fermium doesn't expose SDL_GetWindowFromID
                        // let window_id = WindowId::new(SDL_GetWindowFromID(window_event.windowID));
                        let window_id = WindowId::new(std::ptr::null_mut());
                        match window_event.event {
                            SDL_WINDOWEVENT_CLOSE => {
                                callback(Event::WindowCloseRequested { window_id })
                            }
                            _ => continue,
                        }
                    }
                    _ => continue,
                }
            }
        }
    }
}
