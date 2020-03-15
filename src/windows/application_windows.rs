use super::gl_context_windows::*;
use super::utils_windows::*;
use std::io::Error;
use std::ptr::null_mut;
use winapi::shared::minwindef;
use winapi::shared::windef;
use winapi::um::libloaderapi;
use winapi::um::wingdi;
use winapi::um::winuser;

pub struct Window {
    #[allow(dead_code)]
    handle: windef::HWND,
    device: windef::HDC,
    pub id: WindowId
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct WindowId {
    // This should not be public
    pub handle: windef::HWND, // Just use the window pointer as the ID, it's unique.
}


pub struct WindowBuilder<'a> {
    class_name: Vec<u16>,
    h_instance: minwindef::HINSTANCE,
    x: Option<u32>,
    y: Option<u32>,
    dimensions: Option<(u32, u32)>,
    resizable: bool,
    title: Option<&'a str>,
}

impl<'a> WindowBuilder<'a> {
    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn position(&mut self, x: u32, y: u32) -> &mut Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }
    pub fn dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.dimensions = Some((width, height));
        self
    }

    pub fn build(&self) -> Result<Window, Error> {
        unsafe {
            let extended_style = winuser::WS_EX_APPWINDOW;
            let window_style = winuser::WS_OVERLAPPEDWINDOW | winuser::WS_VISIBLE;
            let title = win32_string(self.title.unwrap_or("Untitled"));

            let x = self.x.map(|x| x as i32).unwrap_or(winuser::CW_USEDEFAULT);
            let y = self.y.map(|y| y as i32).unwrap_or(winuser::CW_USEDEFAULT);

            let (width, height) =
                self.dimensions
                    .map_or((winuser::CW_USEDEFAULT, winuser::CW_USEDEFAULT), |d| {
                        let mut rect = windef::RECT {
                            left: 0,
                            top: 0,
                            right: d.0 as i32,
                            bottom: d.1 as i32,
                        };

                        // Windows will provide a window with a smaller client area than desired (because it includes borders in the window size).
                        // This call returns an adjusted rect accounting for the borders based on the window_style.
                        winuser::AdjustWindowRectEx(
                            &mut rect,
                            window_style,
                            minwindef::FALSE,
                            extended_style,
                        );

                        (rect.right - rect.left, rect.bottom - rect.top)
                    });

            let window_handle = winuser::CreateWindowExW(
                extended_style,
                self.class_name.as_ptr(),
                title.as_ptr(),
                window_style,
                x,
                y,
                width,
                height,
                null_mut(),
                null_mut(),
                self.h_instance,
                null_mut(),
            );
            let window_device = winuser::GetDC(window_handle);
            error_if_null(window_device, false)?;

            Ok(Window {
                handle: window_handle,
                device: window_device,
                id: WindowId {
                    handle: window_handle
                }
            })
        }
    }
}

pub struct ApplicationBuilder {
}

impl ApplicationBuilder {
    pub fn build(&self) -> Result<Application, Error> {
        unsafe {
            // Register the window class.
            let class_name = win32_string("windowing_rust");
            let h_instance = libloaderapi::GetModuleHandleW(null_mut());


            let window_class = winuser::WNDCLASSW {
                style: 0,
                lpfnWndProc: Some(super::event_loop_windows::window_callback),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: h_instance,
                hIcon: null_mut(),
                hCursor: null_mut(), // This may not be what is desired. Potentially this makes it annoying to change the cursor later.
                hbrBackground: null_mut(),
                lpszMenuName: null_mut(),
                lpszClassName: class_name.as_ptr(),
            };
            winuser::RegisterClassW(&window_class);

            Ok(Application {
                class_name,
                h_instance,
            })
        }
    }
}

// This probably shouldn't be possible to clone. 
#[derive( Clone)]
pub struct Application {
    class_name: Vec<u16>,
    h_instance: minwindef::HINSTANCE,
}

impl Application {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder {
         
        }
    }

    pub fn new_window<'a>(&mut self) -> WindowBuilder<'a> {
        WindowBuilder {
            class_name: self.class_name.clone(),
            h_instance: self.h_instance,
            x: None,
            y: None,
            dimensions: None,
            resizable: true,
            title: None,
        }
    }

    
    pub fn event_loop(&mut self) -> EventLoop {
        EventLoop {
        }
    }

    pub fn request_frame(&mut self) {
        unimplemented!()
    }

    pub fn quit(&self) {
        unimplemented!()
    }
}

pub struct EventLoop {
   
}

impl EventLoop {
    pub fn run<T>(&self, callback: T)
    where
        T: 'static + FnMut(crate::Event),
    {
       super::event_loop_windows::run(callback);
    }
}