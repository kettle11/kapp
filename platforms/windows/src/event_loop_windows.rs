use super::WindowId;
/// All windows share a pixel format and an OpenGlContext.
use crate::keys_windows::virtual_keycode_to_key;
use crate::Event;
use crate::Key;
use crate::MouseButton;
use std::ptr::null_mut;
use winapi::shared::minwindef::{HIWORD, LOWORD, LPARAM, LRESULT, TRUE, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::winuser;
use winapi::um::winuser::{MSG, PM_REMOVE, WM_QUIT};

type Callback = dyn 'static + FnMut(Event);
static mut PROGRAM_CALLBACK: Option<Box<Callback>> = None;

pub unsafe extern "system" fn window_callback(
    hwnd: HWND,
    u_msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match u_msg {
        winuser::WM_KEYDOWN => produce_event(process_key_down(w_param, l_param)),
        winuser::WM_KEYUP => produce_event(process_key_up(w_param, l_param)),
        winuser::WM_SIZING => return TRUE as isize,
        winuser::WM_SETCURSOR => {
            // Give the OS a chance to set the cursor first, and don't override it if it sets it.
            // The OS will not set the cursor within a window as the default cursor
            // for a window is set to null.
            if winuser::DefWindowProcW(hwnd, u_msg, w_param, l_param) == 0 {
                winuser::SetCursor(super::application_windows::CURRENT_CURSOR);
            }
            return 0;
        }
        winuser::WM_ENTERSIZEMOVE => {
            return 0;
        }
        winuser::WM_EXITSIZEMOVE => {
            return 0;
        }
        winuser::WM_SIZE => {
            resize_event(hwnd, l_param, w_param);
            produce_event(Event::Draw {
                window_id: WindowId::new(0 as *mut std::ffi::c_void),
            });
            return 0;
        }
        winuser::WM_PAINT => {}
        winuser::WM_LBUTTONDOWN => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::MouseButtonDown {
                x: x as f32,
                y: y as f32,
                button: MouseButton::Left,
            });
        }
        winuser::WM_MBUTTONDOWN => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::MouseButtonDown {
                x: x as f32,
                y: y as f32,
                button: MouseButton::Middle,
            });
        }
        winuser::WM_RBUTTONDOWN => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::MouseButtonDown {
                x: x as f32,
                y: y as f32,
                button: MouseButton::Right,
            });
        }
        winuser::WM_XBUTTONDOWN => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::MouseButtonDown {
                x: x as f32,
                y: y as f32,
                button: match HIWORD(w_param as u32) {
                    winuser::XBUTTON1 => MouseButton::Extra1,
                    winuser::XBUTTON2 => MouseButton::Extra2,
                    _ => unreachable!(),
                },
            });
        }
        winuser::WM_LBUTTONUP => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);

            produce_event(Event::MouseButtonUp {
                x: x as f32,
                y: y as f32,
                button: MouseButton::Left,
            });
        }
        winuser::WM_MBUTTONUP => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);

            produce_event(Event::MouseButtonUp {
                x: x as f32,
                y: y as f32,
                button: MouseButton::Middle,
            });
        }
        winuser::WM_RBUTTONUP => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);

            produce_event(Event::MouseButtonUp {
                x: x as f32,
                y: y as f32,
                button: MouseButton::Right,
            });
        }
        winuser::WM_XBUTTONUP => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::MouseButtonUp {
                x: x as f32,
                y: y as f32,
                button: match HIWORD(w_param as u32) {
                    winuser::XBUTTON1 => MouseButton::Extra1,
                    winuser::XBUTTON2 => MouseButton::Extra2,
                    _ => unreachable!(),
                },
            });
        }
        winuser::WM_MOUSEMOVE => produce_event(process_mouse_move_event(hwnd, l_param)),
        _ => {}
    }
    // DefWindowProcW is the default Window event handler.
    winuser::DefWindowProcW(hwnd, u_msg, w_param, l_param)
}

// https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mousemove
fn resize_event(hwnd: HWND, l_param: LPARAM, w_param: WPARAM) {
    let (width, height) = get_width_height(l_param);
    // First send the resize event
    produce_event(Event::WindowResized {
        width,
        height,
        window_id: WindowId::new(hwnd as *mut std::ffi::c_void),
    });

    // Then send more specific events.
    match w_param {
        winuser::SIZE_MAXIMIZED => produce_event(Event::WindowMaximized {
            window_id: WindowId::new(hwnd as *mut std::ffi::c_void),
        }),
        winuser::SIZE_MINIMIZED => produce_event(Event::WindowMinimized {
            window_id: WindowId::new(hwnd as *mut std::ffi::c_void),
        }),
        winuser::SIZE_RESTORED => {
            /* Quote from the docs: "The window has been resized, but
            neither the SIZE_MINIMIZED nor SIZE_MAXIMIZED value applies" */
            // While resizing the OS directly calls window_callback and does not call the typical event loop.
            // To redraw the window smoothly Event::Draw is passed in here.
            produce_event(Event::WindowRestored {
                window_id: WindowId::new(hwnd as *mut std::ffi::c_void),
            });
            produce_event(Event::Draw {
                window_id: WindowId::new(hwnd as *mut std::ffi::c_void),
            });
        }
        _ => {}
    }

    /*
    produce_event(Event::Draw {
        window_id: WindowId::new(hwnd as *mut std::ffi::c_void),
    });
    */
}

fn produce_event(event: Event) {
    unsafe {
        if let Some(program_callback) = PROGRAM_CALLBACK.as_mut() {
            program_callback(event)
        }
    }
}

// https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mousemove
fn process_mouse_move_event(_hwnd: HWND, l_param: LPARAM) -> Event {
    let x = GET_X_LPARAM(l_param);
    let y = GET_Y_LPARAM(l_param);

    Event::MouseMoved {
        x: x as f32,
        y: y as f32,
    }
}
// https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-size
fn get_width_height(l_param: LPARAM) -> (u32, u32) {
    let width = LOWORD(l_param as u32) as u32;
    let height = HIWORD(l_param as u32) as u32;
    (width, height)
}

fn process_key_down(w_param: WPARAM, l_param: LPARAM) -> Event {
    let (_scancode, key, repeat) = process_key_event(w_param, l_param);

    if repeat {
        Event::KeyRepeat { key }
    } else {
        Event::KeyDown { key }
    }
}

fn process_key_up(w_param: WPARAM, l_param: LPARAM) -> Event {
    let (_scancode, key, _repeat) = process_key_event(w_param, l_param);
    Event::KeyUp { key }
}

fn process_key_event(w_param: WPARAM, l_param: LPARAM) -> (UINT, Key, bool) {
    let scancode = ((l_param >> 16) & 16) as UINT; // bits 16-23 represent the scancode
    let _extended = (l_param >> 24) & 1 != 0; // bit 24 represents if its an extended key
    let repeat = (l_param >> 30) & 1 == 1;
    let key = virtual_keycode_to_key(w_param as _);
    (scancode, key, repeat)
}

pub fn run<T>(callback: T)
where
    T: 'static + FnMut(Event),
{
    unsafe {
        PROGRAM_CALLBACK = Some(Box::new(callback));

        let mut message: MSG = std::mem::zeroed();
        while message.message != WM_QUIT {
            while winuser::PeekMessageW(&mut message, null_mut(), 0, 0, PM_REMOVE) > 0 {
                winuser::TranslateMessage(&message as *const MSG);
                winuser::DispatchMessageW(&message as *const MSG);
            }

            // Issue a draw command after all other events are parsed.
            // TO-DO: This needs to be sent per window
            // TO-DO: Only send if a window requested a redraw

            if let Some(program_callback) = PROGRAM_CALLBACK.as_mut() {
                program_callback(Event::Draw {
                    window_id: WindowId::new(0 as *mut std::ffi::c_void),
                });
            }
        }
    }
}
