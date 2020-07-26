use crate::application_windows::WindowData;
use crate::{
    external_windows::*, keys_windows::virtual_keycode_to_key, Event, Key, PointerButton,
    PointerSource, WindowId,
};
use kapp_platform_common::{event_receiver, redraw_manager};
use std::ptr::null_mut;
pub unsafe extern "system" fn window_callback(
    hwnd: HWND,
    u_msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match u_msg {
        WM_CREATE => {
            // This will be called before any other window functions.
            // Store the WindowData pointer passed as the last parameter in
            // CreateWindowExW
            let data =
                (*(l_param as *mut std::ffi::c_void as *mut CREATESTRUCTA)).lpCreateParams as isize;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, data);
        }
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

            // Redrawing here is required to maintain smooth resizing
            // Resizing does not stretch with VSync on, but is more responsive with VSync off.
            kapp_platform_common::redraw_manager::draw(WindowId::new(
                hwnd as *mut std::ffi::c_void,
            ));

            return 0;
        }
        WM_PAINT => {
            // Drawing here seems to only make resizing more jittery.
            // So don't bother
        }
        WM_LBUTTONDOWN => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::PointerDown {
                x: x as f64,
                y: y as f64,
                source: PointerSource::Mouse,
                button: PointerButton::Primary,
                timestamp: get_message_time(),
            });
        }
        WM_MBUTTONDOWN => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::PointerDown {
                x: x as f64,
                y: y as f64,
                source: PointerSource::Mouse,
                button: PointerButton::Auxillary,
                timestamp: get_message_time(),
            });
        }
        WM_RBUTTONDOWN => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::PointerDown {
                x: x as f64,
                y: y as f64,
                source: PointerSource::Mouse,
                button: PointerButton::Secondary,
                timestamp: get_message_time(),
            });
        }
        WM_XBUTTONDOWN => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::PointerDown {
                x: x as f64,
                y: y as f64,
                source: PointerSource::Mouse,
                button: match HIWORD(w_param as u32) {
                    XBUTTON1 => PointerButton::Extra1,
                    XBUTTON2 => PointerButton::Extra2,
                    _ => unreachable!(),
                },
                timestamp: get_message_time(),
            });
        }
        WM_LBUTTONUP => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);

            produce_event(Event::PointerUp {
                x: x as f64,
                y: y as f64,
                source: PointerSource::Mouse,
                button: PointerButton::Primary,
                timestamp: get_message_time(),
            });
        }
        WM_MBUTTONUP => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);

            produce_event(Event::PointerUp {
                x: x as f64,
                y: y as f64,
                source: PointerSource::Mouse,
                button: PointerButton::Auxillary,
                timestamp: get_message_time(),
            });
        }
        WM_RBUTTONUP => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);

            produce_event(Event::PointerUp {
                x: x as f64,
                y: y as f64,
                source: PointerSource::Mouse,
                button: PointerButton::Secondary,
                timestamp: get_message_time(),
            });
        }
        WM_XBUTTONUP => {
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::PointerUp {
                x: x as f64,
                y: y as f64,
                source: PointerSource::Mouse,
                button: match HIWORD(w_param as u32) {
                    XBUTTON1 => PointerButton::Extra1,
                    XBUTTON2 => PointerButton::Extra2,
                    _ => unreachable!(),
                },
                timestamp: get_message_time(),
            });
        }
        WM_MOUSEMOVE => produce_event(process_mouse_move_event(hwnd, l_param)),
        WM_LBUTTONDBLCLK => {
            // When double click is enabled on a window Windows will consume the second down event
            // so send a synthetic one here to ensure that no clicks are missed.
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::PointerDown {
                x: x as f64,
                y: y as f64,
                source: PointerSource::Mouse,
                button: PointerButton::Primary,
                timestamp: get_message_time(),
            });
            produce_event(Event::DoubleClickDown {
                x: x as f64,
                y: y as f64,
                button: PointerButton::Primary,
                timestamp: get_message_time(),
            });
        }
        WM_MBUTTONDBLCLK => {
            // When double click is enabled on a window Windows will consume the second down event
            // so send a synthetic one here to ensure that no clicks are missed.
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::PointerDown {
                x: x as f64,
                y: y as f64,
                source: PointerSource::Mouse,
                button: PointerButton::Auxillary,
                timestamp: get_message_time(),
            });
            produce_event(Event::DoubleClickDown {
                x: x as f64,
                y: y as f64,
                button: PointerButton::Auxillary,
                timestamp: get_message_time(),
            });
        }
        WM_RBUTTONDBLCLK => {
            // When double click is enabled on a window Windows will consume the second down event
            // so send a synthetic one here to ensure that no clicks are missed.
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            produce_event(Event::PointerDown {
                x: x as f64,
                y: y as f64,
                source: PointerSource::Mouse,
                button: PointerButton::Secondary,
                timestamp: get_message_time(),
            });
            produce_event(Event::DoubleClickDown {
                x: x as f64,
                y: y as f64,
                button: PointerButton::Secondary,
                timestamp: get_message_time(),
            });
        }
        WM_XBUTTONDBLCLK => {
            // When double click is enabled on a window Windows will consume the second down event
            // so send a synthetic one here to ensure that no clicks are missed.
            let x = GET_X_LPARAM(l_param);
            let y = GET_Y_LPARAM(l_param);
            let button = match HIWORD(w_param as u32) {
                XBUTTON1 => PointerButton::Extra1,
                XBUTTON2 => PointerButton::Extra2,
                _ => unreachable!(),
            };

            produce_event(Event::PointerDown {
                x: x as f64,
                y: y as f64,
                source: PointerSource::Mouse,
                button,
                timestamp: get_message_time(),
            });
            produce_event(Event::DoubleClickDown {
                x: x as f64,
                y: y as f64,
                button,
                timestamp: get_message_time(),
            });
        }
        WM_DPICHANGED => {
            let scale_dpi_width = LOWORD(w_param as u32) as u32;
            // USER_DEFAULT_SCREEN_DPI is 96.
            // 96 is considered the default scale.
            let scale = scale_dpi_width as f64 / USER_DEFAULT_SCREEN_DPI as f64;
            produce_event(Event::WindowScaleChanged {
                scale,
                window_id: WindowId::new(hwnd as *mut std::ffi::c_void),
            });
        }
        WM_GETMINMAXINFO => {
            // This is the first message sent to a new window.
            // But it does not seem like it's safe to assume that.

            // It is not safe to assume that window data is initialized yet.
            let window_data = get_window_data(hwnd);
            if let Some(window_data) = window_data {
                let min_max_info = l_param as *mut MINMAXINFO;
                (*min_max_info).ptMinTrackSize.x = (*window_data).minimum_width as i32;
                (*min_max_info).ptMinTrackSize.y = (*window_data).minimum_height as i32;
            }
        }
        WM_NCDESTROY => {
            // Deallocate data associated with this window.
            let _ = Box::from_raw(GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowData);
        }
        _ => {}
    }
    // DefWindowProcW is the default Window event handler.
    DefWindowProcW(hwnd, u_msg, w_param, l_param)
}

fn get_window_data(hwnd: HWND) -> Option<*mut WindowData> {
    let data = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowData };
    if data == std::ptr::null_mut() {
        None
    } else {
        Some(data)
    }
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
            kapp_platform_common::redraw_manager::draw(WindowId::new(
                hwnd as *mut std::ffi::c_void,
            ));
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

    Event::PointerMoved {
        x: x as f64,
        y: y as f64,
        source: PointerSource::Mouse,
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

        while message.message != WM_QUIT {
            // Block and wait for messages unless there is a redraw request.
            // GetMessageW will return 0 if WM_QUIT is encountered
            while redraw_manager::draw_requests_count() == 0
                && GetMessageW(&mut message, null_mut(), 0, 0) > 0
            {
                TranslateMessage(&message as *const MSG);
                DispatchMessageW(&message as *const MSG);
            }

            if message.message == WM_QUIT {
                break;
            }

            // We only reach here if there are redraw requests.
            // Iterate through all messages without blocking.
            while PeekMessageW(&mut message, null_mut(), 0, 0, PM_REMOVE) > 0 {
                TranslateMessage(&message as *const MSG);
                DispatchMessageW(&message as *const MSG);
            }

            redraw_manager::begin_draw_flush();
            while let Some(window_id) = redraw_manager::get_draw_request() {
                produce_event(Event::Draw { window_id });
            }
            // Need to rerun event loop here if there are any redraw requests.
        }
    }
}
