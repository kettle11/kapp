use super::apple::*;
use crate::AppParameters;
use crate::Event;
use crate::WindowParameters;
use std::io::Error;

pub const nil: *mut Object = 0 as *mut Object;

type Callback = dyn 'static + FnMut(Event, &mut App);
static mut PROGRAM_CALLBACK: Option<Box<Callback>> = None;
static mut APP: Option<Box<App>> = None;

struct ViewData {
    view: *mut Object,
}
static mut TEST_VIEW: Option<Box<ViewData>> = None;

pub struct Window {
    view: *mut Object,
}

fn produce_event(event: crate::Event) {
    unsafe {
        if let Some(program_callback) = PROGRAM_CALLBACK.as_mut() {
            if let Some(app) = APP.as_mut() {
                program_callback(event, app);
            }
        }
    }
}

#[derive(Clone)]
pub struct App {
    pub app: *mut Object,
    gl_context: *mut Object,
    window_delegate_class: *const objc::runtime::Class,
    view_delegate_class: *const objc::runtime::Class,
}

fn get_window_data(this: &Object) -> *mut Object {
    unsafe {
        let data = *this.get_ivar("window_data");
        println!("WINDOW OUT:{:?}", data);
        data
    }
}
impl App {
    pub fn new(app_parameters: &AppParameters) -> Result<App, ()> {
        unsafe {
            // let pool: *mut Object = unsafe { msg_send![class!(NSAutoreleasePool), new] };

            let app: *mut Object = msg_send![class!(NSApplication), sharedApplication];
            let () = msg_send![
                app,
                setActivationPolicy:
                    NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular
            ];

            let attrs = [
                NSOpenGLPFAOpenGLProfile as u32,
                NSOpenGLProfileVersion3_2Core as u32, // Needed if using opengl 3.2 you can comment this line out to use the old version.
                NSOpenGLPFAColorSize as u32,
                24,
                NSOpenGLPFAAlphaSize as u32,
                8,
                NSOpenGLPFADepthSize as u32,
                24,
                NSOpenGLPFAStencilSize as u32,
                8,
                NSOpenGLPFAAccelerated as u32,
                NSOpenGLPFADoubleBuffer as u32,
                0,
            ];

            let pixel_format: *mut Object = msg_send![class!(NSOpenGLPixelFormat), alloc];
            let pixel_format: *mut Object = msg_send![pixel_format, initWithAttributes: &attrs];

            let gl_context: *mut Object = msg_send![class!(NSOpenGLContext), alloc];
            let gl_context: *mut Object =
                msg_send![gl_context, initWithFormat: pixel_format shareContext: nil];
            let () = msg_send![gl_context, makeCurrentContext];

            // Enable vsync
            let () = msg_send![gl_context, setValues:&(1 as i32) forParameter:NSOpenGLContextParameter::NSOpenGLCPSwapInterval];

            // Setup window_delegate_class
            let superclass = class!(NSResponder);
            let mut decl = ClassDecl::new("KettlewinWindowClass", superclass).unwrap();
            extern "C" fn window_moved(this: &Object, _sel: Sel, event: *mut Object) {
                //  println!("WINDOW MOVED");
                /*
                unsafe {
                    if let Some(data) = TEST_VIEW.as_ref() {
                        let () = msg_send![data.view, setNeedsDisplay: YES];
                    }
                }*/
            }
            extern "C" fn window_did_resize(this: &Object, _sel: Sel, event: *mut Object) {
                // TEST_VIEW needs to be replaced with the actual window view.
                unsafe {
                    if let Some(data) = TEST_VIEW.as_ref() {
                        let rect: NSRect = msg_send![data.view, frame];
                        let window = get_window_data(this);
                        let new_name = NSString::new("resized");
                        let () = msg_send![window, setTitle: new_name.raw];

                        let screen: *mut Object = msg_send![window, screen];

                        let scale_factor: CGFloat = msg_send![screen, backingScaleFactor];

                        println!("Backing scale factor: {:?}", scale_factor);

                        println!("RECT: {:?}", rect);
                        let width = rect.size.width;
                        let height = rect.size.height;

                        println!(
                            "RESIZED SCALED: {:?}, {:?}",
                            width * scale_factor,
                            height * scale_factor
                        );
                        produce_event(crate::Event::ResizedWindow {
                            width: width as u32,
                            height: height as u32,
                        });
                    }
                }
            }

            decl.add_method(
                sel!(windowDidMove:),
                window_moved as extern "C" fn(&Object, Sel, *mut Object),
            );
            decl.add_method(
                sel!(windowDidResize:),
                window_did_resize as extern "C" fn(&Object, Sel, *mut Object),
            );

            decl.add_ivar::<*mut Object>("window_data");

            let window_delegate_class = decl.register();

            // Setup view_delegate_class
            let superclass = class!(NSView);
            let mut decl = ClassDecl::new("KettlewinViewClass", superclass).unwrap();
            extern "C" fn key_down(this: &Object, _sel: Sel, event: *mut Object) {
                unsafe {
                    let key_code = msg_send![event, keyCode];
                    produce_event(crate::Event::ButtonDown {
                        button: super::keys_mac::virtual_keycode_to_key(key_code),
                        scancode: 0,
                    });
                }
            }

            extern "C" fn key_up(this: &Object, _sel: Sel, event: *mut Object) {
                unsafe {
                    let key_code = msg_send![event, keyCode];
                    produce_event(crate::Event::ButtonUp {
                        button: super::keys_mac::virtual_keycode_to_key(key_code),
                        scancode: 0,
                    });
                }
            }
            extern "C" fn draw_rect(this: &Object, _sel: Sel, rect: NSRect) {}

            extern "C" fn mouse_moved(this: &Object, _sel: Sel, event: *mut Object) {
                unsafe {
                    // The following code snippet is taken from winit.
                    // We have to do this to have access to the `NSView` trait...
                    let view: *mut Object = this as *const _ as *mut _;

                    let window_point: NSPoint = msg_send![event, locationInWindow];
                    let view_point: NSPoint =
                        msg_send![view, convertPoint: window_point fromView: nil];
                    let view_rect: NSRect = msg_send![this, frame];

                    if view_point.x.is_sign_negative()
                        || view_point.y.is_sign_negative()
                        || view_point.x > view_rect.size.width
                        || view_point.y > view_rect.size.height
                    {
                        // Point is outside of the client area (view)
                        return;
                    }

                    let x = view_point.x;
                    let y = view_rect.size.height - view_point.y;

                    produce_event(crate::Event::MouseMoved {
                        x: x as f32,
                        y: y as f32,
                    });
                }
            }

            decl.add_method(
                sel!(mouseMoved:),
                mouse_moved as extern "C" fn(&Object, Sel, *mut Object),
            );
            decl.add_method(
                sel!(keyDown:),
                key_down as extern "C" fn(&Object, Sel, *mut Object),
            );
            decl.add_method(
                sel!(keyUp:),
                key_up as extern "C" fn(&Object, Sel, *mut Object),
            );

            decl.add_method(
                sel!(drawRect:),
                draw_rect as extern "C" fn(&Object, Sel, NSRect),
            );

            let view_delegate_class = decl.register();

            // Stuff taken from Winit to setup a loopobserver
            extern "C" fn control_flow_end_handler(
                _: CFRunLoopObserverRef,
                activity: CFRunLoopActivity,
                _: *mut std::ffi::c_void,
            ) {
                produce_event(Event::Draw);
            }
            // Setup a runloop observer (Idea borrowed from Winit)
            let observer = CFRunLoopObserverCreate(
                std::ptr::null_mut(),
                kCFRunLoopBeforeWaiting,
                YES,                  // Indicates we want this to run repeatedly
                CFIndex::min_value(), // The lower the value, the sooner this will run
                control_flow_end_handler,
                std::ptr::null_mut(),
            );
            CFRunLoopAddObserver(CFRunLoopGetMain(), observer, kCFRunLoopCommonModes);

            // This event is empty because it's only used continuously wakeup the main thread.
            extern "C" fn wakeup_main_loop(
                _timer: CFRunLoopTimerRef,
                _info: *mut std::ffi::c_void,
            ) {
            }
            let timer = CFRunLoopTimerCreate(
                std::ptr::null_mut(),
                0.,
                0.000_000_1,
                0,
                0,
                wakeup_main_loop,
                std::ptr::null_mut(),
            );
            CFRunLoopAddTimer(CFRunLoopGetMain(), timer, kCFRunLoopCommonModes);

            Ok(App {
                app,
                gl_context,
                window_delegate_class,
                view_delegate_class,
            })
        }
    }

    fn setup_opengl() -> Result<(), Error> {
        Ok(())
    }

    pub fn new_window<'a>(&mut self, window_parameters: &WindowParameters) -> Result<Window, ()> {
        unsafe {
            let (x, y) = window_parameters
                .position
                .map_or((0., 0.), |(x, y)| (x as f64, y as f64));

            let (width, height) = window_parameters
                .dimensions
                .map_or((600., 600.), |(width, height)| {
                    (width as f64, height as f64)
                });
            let rect = NSRect::new(NSPoint::new(x, y), NSSize::new(width, height));

            // It appears these flags are deprecated, but the Rust wrapper does not expose the nondepcrated version?
            let mut style = NSWindowStyleMaskTitled
                | NSWindowStyleMaskClosable
                | NSWindowStyleMaskMiniaturizable;
            if window_parameters.resizable {
                style |= NSWindowStyleMaskResizable;
            }

            // Needs to be released somehow

            let window: *mut Object = msg_send![class!(NSWindow), alloc];
            let () = msg_send![
                window,
                initWithContentRect:rect
                styleMask:style
                backing:NSBackingStoreBuffered
                defer:NO
            ];

            let () = msg_send![window, cascadeTopLeftFromPoint:NSPoint::new(20., 20.)];

            let () = msg_send![window, center];
            let title = window_parameters.title.unwrap_or("Untitled");
            let title = NSString::new(title);
            let () = msg_send![window, setTitle: title.raw];
            let () = msg_send![window, makeKeyAndOrderFront: nil];

            // setup view
            let view: *mut Object = msg_send![self.view_delegate_class, alloc];
            // Apparently this defaults to YES even without this call
            let () = msg_send![view, setWantsBestResolutionOpenGLSurface: YES];

            // Setup a tracking area to receive mouse events within
            let tracking_area: *mut Object = msg_send![class!(NSTrackingArea), alloc];
            let () = msg_send![
                tracking_area,
                initWithRect: rect
                options: NSTrackingMouseEnteredAndExited | NSTrackingMouseMoved | NSTrackingActiveInKeyWindow
                owner: view
                userInfo:nil];
            let () = msg_send![view, addTrackingArea: tracking_area];

            // setup window delegate
            let window_delegate: *mut Object = msg_send![self.window_delegate_class, new];

            (*window_delegate).set_ivar("window_data", window);
            println!("WINDOW IN:{:?}", window);
            let () = msg_send![window, setDelegate: window_delegate];
            let () = msg_send![window, setContentView: view];
            let () = msg_send![window, makeFirstResponder: view];

            TEST_VIEW = Some(Box::new(ViewData { view }));

            let window = Window { view };
            self.make_current(&window).unwrap();
            Ok(window)
        }
    }

    pub fn make_current(&self, window: &Window) -> Result<(), Error> {
        unsafe {
            let () = msg_send![self.gl_context, setView: window.view];
        }
        Ok(())
    }

    pub fn swap_buffers(&self) {
        unsafe {
            let () = msg_send![self.gl_context, flushBuffer];
        }
    }

    // This belongs to the window builder because the OpenGL context must be constructed first
    // and the window builder creates the context.
    pub fn gl_loader(&self) -> Box<dyn FnMut(&'static str) -> *const std::ffi::c_void> {
        unimplemented!();
    }

    pub fn gl_loader_c_string(&self) -> Box<dyn FnMut(*const i8) -> *const std::ffi::c_void> {
        unsafe {
            Box::new(move |s| unsafe {
                let name = std::ffi::CStr::from_ptr(s);
                get_proc_address((&name).to_str().unwrap())
            })
        }
    }

    #[cfg(feature = "opengl_glow")]
    pub fn gl_context(&self) -> glow::Context {
        glow::Context::from_loader_function(|s| get_proc_address(s))
    }

    pub fn run<T>(&mut self, mut callback: T)
    where
        T: 'static + FnMut(crate::Event, &mut App),
    {
        println!("Running");
        unsafe {
            PROGRAM_CALLBACK = Some(Box::new(callback));
            APP = Some(Box::new(self.clone()));
            let () = msg_send![self.app, run];
        }
    }
}

// Taken from Glutin:
// https://github.com/rust-windowing/glutin/blob/447f3526dcf90a460d52afefd0b29eb2ed7f87f3/glutin/src/platform_impl/macos/mod.rs
fn get_proc_address(addr: &str) -> *const core::ffi::c_void {
    let symbol_name = NSString::new(addr);
    let framework_name = NSString::new("com.apple.opengl");
    let framework = unsafe { CFBundleGetBundleWithIdentifier(framework_name.raw) };
    let symbol = unsafe { CFBundleGetFunctionPointerForName(framework, symbol_name.raw) };
    symbol as *const _
}
