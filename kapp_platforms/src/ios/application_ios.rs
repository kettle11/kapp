use super::uikit::*;
use kapp_platform_common::*;
use std::ffi::CString;
pub struct PlatformEventLoop {}

impl PlatformEventLoopTrait for PlatformEventLoop {
    fn run(&self, callback: Box<dyn FnMut(Event)>) {
        // This call does not return.
        unsafe {
            UIApplicationMain(
                0,
                std::ptr::null_mut(),
                nil,
                NSString::new("kAppAppDelegate").raw,
            );
        }
    }
}

fn application_delegate_declaration() -> *const objc::runtime::Class {
    let superclass = class!(UIResponder);
    let mut decl = ClassDecl::new("kAppAppDelegate", superclass).unwrap();
    super::events_ios::add_application_events_to_decl(&mut decl);
    decl.register()
}

pub struct PlatformApplication {}

impl PlatformApplicationTrait for PlatformApplication {
    type EventLoop = PlatformEventLoop;

    fn new() -> Self {
        application_delegate_declaration();
        PlatformApplication {}
    }

    fn event_loop(&mut self) -> Self::EventLoop {
        PlatformEventLoop {}
    }

    fn set_window_position(&mut self, window_id: WindowId, x: u32, y: u32) {
        // No analogous behavior
    }

    fn set_window_size(&mut self, window_id: WindowId, width: u32, height: u32) {
        // No analogous behavior
    }

    fn set_window_title(&mut self, window_id: WindowId, title: &str) {
        // No analogous behavior?
    }

    fn minimize_window(&mut self, window_id: WindowId) {
        // No analogous behavior
    }

    fn maximize_window(&mut self, _window_id: WindowId) {
        // No analogous behavior?
    }

    fn fullscreen_window(&mut self, window_id: WindowId) {
        unimplemented!();
    }

    fn restore_window(&mut self, _window_id: WindowId) {
        unimplemented!()
    }

    fn close_window(&mut self, window_id: WindowId) {
        // No analogous behavior?
    }

    fn redraw_window(&mut self, window_id: WindowId) {
        unimplemented!();
    }

    fn get_window_size(&mut self, window_id: WindowId) -> (u32, u32) {
        let frame: CGRect = unsafe { msg_send![window_id.raw() as *mut Object, getFrame] };
        (frame.size.width as u32, frame.size.height as u32)
    }

    fn get_window_scale(&mut self, window_id: WindowId) -> f64 {
        unimplemented!();
    }

    fn set_mouse_position(&mut self, _x: u32, _y: u32) {
        unimplemented!();
    }

    // https://developer.apple.com/documentation/appkit/nscursor?language=objc
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
            // Get the main screen and set the window to be the size of the main screen.
            let main_screen: *mut Object = msg_send![class!(UIScreen), mainScreen];
            let main_screen_bounds: CGRect = msg_send![main_screen, bounds];
            let ui_window: *mut Object = msg_send![class!(UIWindow), alloc];
            let ui_window: *mut Object = msg_send![ui_window, initWithFrame: main_screen_bounds];
            WindowId::new(ui_window as *mut std::ffi::c_void)
        }
    }

    fn quit(&self) {
        // No analogous behavior
    }

    fn raw_window_handle(&self, window_id: WindowId) -> RawWindowHandle {
        let ui_window = unsafe { window_id.raw() };
        raw_window_handle::RawWindowHandle::IOS(raw_window_handle::ios::IOSHandle {
            ui_window,
            ..raw_window_handle::ios::IOSHandle::empty()
        })
    }
}

// When the application is dropped, quit the program.
impl Drop for PlatformApplication {
    fn drop(&mut self) {
        self.quit();
    }
}
