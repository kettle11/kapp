use super::WindowId;
use crate::external_windows::*;
/// All windows share a pixel format and an OpenGlContext.
use crate::keys_windows::virtual_keycode_to_key;
use crate::Event;
use crate::Key;
use crate::MouseButton;
use kapp_platform_common::event_receiver;
use std::ptr::null_mut;

pub unsafe extern "system" fn window_callback(
    hwnd: HWND,
    u_msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match u_msg {
        WM_CLOSE => {
            produce_event(Event::WindowCloseRequested {
                window_id: WindowId::new(hwnd as *mut std::ffi::c_void),
            });
            // Return 0 to reject the close because the user application must approve the close.
            return 0;
        }
        WM_KEYDOWN => produce_event(process_key_down(w_param, l_param)),
        WM_KEYUP => produce_event(process_key_up(w_param, l_param)),
        WM_SIZING => return TRUE as isize,
        WM_SETCURSOR => {
            // Give the OS a chance to set the cursor first, and don't override it if it sets it.
            // The OS will not set the cursor within a window as the default cursor
            // for a window is set to null.
            if DefWindowProcW(hwnd, u_msg, w_param, l_param) == 0 {
                SetCursor(super::application_windows::CURRENT_CURSOR);
            }
            return 0;
        }
        WM_ENTERSIZEMOVE => {
            return 0;
        }
        WM_EXITSIZEMOVE => {
            return 0;
        }
        WM_SIZE => {
            resize_event(hwnd, l_param, w_param);
            produce_event(Event::Draw {
                window_id: WindowId::new(hwnd as *mut std::ffi::c_void),
            });
            return 0;
        }
        WM_PAINT => {}
        WM_LBUTTONDOWN => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::MouseButtonDown {
                x: x as f32,
                y: y as f32,
                timestamp: get_message_time(),
                button: MouseButton::Left,
            });
        }
        WM_MBUTTONDOWN => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::MouseButtonDown {
                x: x as f32,
                y: y as f32,
                timestamp: get_message_time(),
                button: MouseButton::Middle,
            });
        }
        WM_RBUTTONDOWN => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::MouseButtonDown {
                x: x as f32,
                y: y as f32,
                timestamp: get_message_time(),
                button: MouseButton::Right,
            });
        }
        WM_XBUTTONDOWN => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::MouseButtonDown {
                x: x as f32,
                y: y as f32,
                timestamp: get_message_time(),
                button: match HIWORD(w_param as u32) {
                    XBUTTON1 => MouseButton::Extra1,
                    XBUTTON2 => MouseButton::Extra2,
                    _ => unreachable!(),
                },
            });
        }
        WM_LBUTTONUP => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);

            produce_event(Event::MouseButtonUp {
                x: x as f32,
                y: y as f32,
                timestamp: get_message_time(),
                button: MouseButton::Left,
            });
        }
        WM_MBUTTONUP => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);

            produce_event(Event::MouseButtonUp {
                x: x as f32,
                y: y as f32,
                timestamp: get_message_time(),
                button: MouseButton::Middle,
            });
        }
        WM_RBUTTONUP => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);

            produce_event(Event::MouseButtonUp {
                x: x as f32,
                y: y as f32,
                timestamp: get_message_time(),
                button: MouseButton::Right,
            });
        }
        WM_XBUTTONUP => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::MouseButtonUp {
                x: x as f32,
                y: y as f32,
                timestamp: get_message_time(),
                button: match HIWORD(w_param as u32) {
                    XBUTTON1 => MouseButton::Extra1,
                    XBUTTON2 => MouseButton::Extra2,
                    _ => unreachable!(),
                },
            });
        }
        WM_MOUSEMOVE => produce_event(process_mouse_move_event(hwnd, l_param)),
        _ => {}
    }
    // DefWindowProcW is the default Window event handler.
    DefWindowProcW(hwnd, u_msg, w_param, l_param)
}

/// Gets the message time with millisecond precision
/// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagetime
fn get_message_time() -> std::time::Duration {
    std::time::Duration::from_millis(unsafe { GetMessageTime() } as u64)
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
        SIZE_MAXIMIZED => produce_event(Event::WindowMaximized {
            window_id: WindowId::new(hwnd as *mut std::ffi::c_void),
        }),
        SIZE_MINIMIZED => produce_event(Event::WindowMinimized {
            window_id: WindowId::new(hwnd as *mut std::ffi::c_void),
        }),
        SIZE_RESTORED => {
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
}

fn produce_event(event: Event) {
    event_receiver::send_event(event);
}

// https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mousemove
fn process_mouse_move_event(_hwnd: HWND, l_param: LPARAM) -> Event {
    let x = GET_X_LPARAM(l_param);
    let y = GET_Y_LPARAM(l_param);

    Event::MouseMoved {
        x: x as f32,
        y: y as f32,
        timestamp: get_message_time(),
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
        Event::KeyRepeat {
            key,
            timestamp: get_message_time(),
        }
    } else {
        Event::KeyDown {
            key,
            timestamp: get_message_time(),
        }
    }
}

fn process_key_up(w_param: WPARAM, l_param: LPARAM) -> Event {
    let (_scancode, key, _repeat) = process_key_event(w_param, l_param);
    Event::KeyUp {
        key,
        timestamp: get_message_time(),
    }
}

fn process_key_event(w_param: WPARAM, l_param: LPARAM) -> (UINT, Key, bool) {
    let scancode = ((l_param >> 16) & 16) as UINT; // bits 16-23 represent the scancode
    let _extended = (l_param >> 24) & 1 != 0; // bit 24 represents if its an extended key
    let repeat = (l_param >> 30) & 1 == 1;
    let key = virtual_keycode_to_key(w_param as _);
    (scancode, key, repeat)
}

pub fn run(callback: Box<dyn FnMut(crate::Event)>) {
    unsafe {
        event_receiver::set_callback(callback);

        let mut message: MSG = std::mem::zeroed();

        let mut temp_draw_request_buffer = Vec::new();
        while message.message != WM_QUIT {
            while PeekMessageW(&mut message, null_mut(), 0, 0, PM_REMOVE) > 0 {
                TranslateMessage(&message as *const MSG);
                DispatchMessageW(&message as *const MSG);
            }

            // The draw request buffer cannot be the same as the one read below
            // because otherwise requesting a draw creates an infinite loop.
            std::mem::swap(
                &mut temp_draw_request_buffer,
                &mut super::application_windows::WINDOWS_TO_REDRAW,
            );

            while let Some(window_id) = &temp_draw_request_buffer.pop() {
                produce_event(Event::Draw {
                    window_id: *window_id,
                });
            }
        }
    }
}
