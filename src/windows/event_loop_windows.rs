/// All windows share a pixel format and an OpenGlContext.
use super::keys_windows::virtual_keycode_to_key;
use crate::events::*;
use crate::Key;
use std::ptr::null_mut;
use winapi::shared::minwindef::{HIWORD, LOWORD, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::um::winuser;
use winapi::um::winuser::{MSG, PM_REMOVE, WM_QUIT};

type Callback = dyn 'static + FnMut(Event);
static mut PROGRAM_CALLBACK: Option<Box<Callback>> = None;

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

            if let Some(program_callback) = PROGRAM_CALLBACK.as_mut() {
                program_callback(Event::Draw);
            }
        }
    }
}

pub unsafe extern "system" fn window_callback(
    hwnd: HWND,
    u_msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match u_msg {
        winuser::WM_KEYDOWN => produce_event(process_key_down(w_param, l_param)),
        winuser::WM_KEYUP => produce_event(process_key_up(w_param, l_param)),
        winuser::WM_SIZE => {
            let (width, height) = get_width_height(l_param);
            // First send the resize event
            produce_event(Event::ResizedWindow { width, height });

            // Then send more specific events.
            match w_param {
                winuser::SIZE_MAXIMIZED => produce_event(Event::MaximizedWindow),
                winuser::SIZE_MINIMIZED => produce_event(Event::MinimizedWindow),
                winuser::SIZE_RESTORED => {
                    /* Quote from the docs: "The window has been resized, but
                    neither the SIZE_MINIMIZED nor SIZE_MAXIMIZED value applies" */
                    // While resizing the OS directly calls window_callback and does not call the typical event loop.
                    // To redraw the window smoothly Event::Draw is passed in here.
                    produce_event(Event::Draw);
                }
                _ => {}
            }
        }
        winuser::WM_LBUTTONDOWN => produce_event(Event::MouseDown {
            button: MouseButton::Left,
        }),
        winuser::WM_MBUTTONDOWN => produce_event(Event::MouseDown {
            button: MouseButton::Middle,
        }),
        winuser::WM_RBUTTONDOWN => produce_event(Event::MouseDown {
            button: MouseButton::Right,
        }),
        winuser::WM_LBUTTONUP => produce_event(Event::MouseUp {
            button: MouseButton::Left,
        }),
        winuser::WM_MBUTTONUP => produce_event(Event::MouseUp {
            button: MouseButton::Middle,
        }),
        winuser::WM_RBUTTONUP => produce_event(Event::MouseUp {
            button: MouseButton::Right,
        }),
        winuser::WM_MOUSEMOVE => produce_event(process_mouse_move_event(hwnd, l_param)),
        _ => {}
    }
    // DefWindowProcW is the default Window event handler.
    winuser::DefWindowProcW(hwnd, u_msg, w_param, l_param)
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
    let (scancode, key) = process_key_event(w_param, l_param);
    Event::KeyDown { key, scancode }
}

fn process_key_up(w_param: WPARAM, l_param: LPARAM) -> Event {
    let (scancode, key) = process_key_event(w_param, l_param);
    Event::KeyUp { key, scancode }
}

fn process_key_event(w_param: WPARAM, l_param: LPARAM) -> (UINT, Key) {
    let scancode = ((l_param >> 16) & 16) as UINT; // bits 16-23 represent the scancode
    let _extended = (l_param & (1 << 24)) != 0; // bit 24 represents if its an extended key
    let key = virtual_keycode_to_key(w_param as _);
    (scancode, key)
}
