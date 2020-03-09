use std::io::Error;
extern crate cocoa;
extern crate core_foundation;

use crate::Event;
use cocoa::appkit::*;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSInteger, NSPoint, NSRect, NSSize, NSString};
use core_foundation::base::TCFType;
use core_foundation::bundle::{CFBundleGetBundleWithIdentifier, CFBundleGetFunctionPointerForName};
use core_foundation::string::CFString;

use objc::{
    declare::ClassDecl,
    runtime::{Class, Object, Sel, BOOL, NO, YES},
};

use std::str::FromStr;

type Callback = dyn 'static + FnMut(Event, &mut App);
static mut PROGRAM_CALLBACK: Option<Box<Callback>> = None;
static mut APP: Option<Box<App>> = None;

struct ViewData {
    view: id,
}
static mut TEST_VIEW: Option<Box<ViewData>> = None;

pub struct Window {
    view: id,
}

pub struct WindowBuilder<'a> {
    position: Option<(u32, u32)>,
    dimensions: Option<(u32, u32)>,
    resizable: bool,
    title: Option<&'a str>,
}

impl<'a> WindowBuilder<'a> {
    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn resizable(&mut self, resizable: bool) -> &mut Self {
        self.resizable = resizable;
        self
    }

    pub fn position(&mut self, x: u32, y: u32) -> &mut Self {
        self.position = Some((x, y));
        self
    }
    pub fn dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.dimensions = Some((width, height));
        self
    }

    pub fn build(&self, app: &App) -> Result<Window, Error> {
        unsafe {
            let (x, y) = self
                .position
                .map_or((0., 0.), |(x, y)| (x as f64, y as f64));

            let (width, height) = self.dimensions.map_or((600., 600.), |(width, height)| {
                (width as f64, height as f64)
            });
            let rect = NSRect::new(NSPoint::new(x, y), NSSize::new(width, height));

            // It appears these flags are deprecated, but the Rust wrapper does not expose the nondepcrated version?
            let mut style = NSWindowStyleMask::NSTitledWindowMask
                | NSWindowStyleMask::NSClosableWindowMask
                | NSWindowStyleMask::NSMiniaturizableWindowMask;
            if self.resizable {
                style |= NSWindowStyleMask::NSResizableWindowMask;
            }

            let window = NSWindow::alloc(nil)
                .initWithContentRect_styleMask_backing_defer_(
                    rect,
                    style,
                    NSBackingStoreBuffered,
                    NO,
                )
                .autorelease();
            window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
            window.center();
            let title = self.title.unwrap_or("Untitled");
            let title = NSString::alloc(nil).init_str(title);
            cocoa::appkit::NSWindow::setTitle_(window, title);
            window.makeKeyAndOrderFront_(nil);
            let current_app = NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
            // setup window delegate
            let window_delegate: id = msg_send![app.window_delegate_class, new];
            let () = msg_send![window, setDelegate: window_delegate];

            // setup view
            let view: id = msg_send![app.view_delegate_class, new];
            let () = msg_send![window, setDelegate: window_delegate];
            let () = msg_send![window, setContentView: view];
            let () = msg_send![window, makeFirstResponder: view];

            // Setup a tracking rect to receive mouse events within
            let rect: NSRect = msg_send![view, visibleRect];
            let tracking_rect: NSInteger = msg_send![view,
                addTrackingRect:rect
                owner:view
                userData:nil
                assumeInside:NO
            ];

            let trackingArea: id = msg_send![class!(NSTrackingArea), alloc];
            let () = msg_send![
                trackingArea,
                initWithRect: rect
                options: NSTrackingMouseEnteredAndExited | NSTrackingMouseMoved | NSTrackingActiveInKeyWindow
                owner: view
                userInfo:nil];
            let () = msg_send![view, addTrackingArea: trackingArea];
            TEST_VIEW = Some(Box::new(ViewData { view }));
            let window = Window { view };
            app.make_current(&window).unwrap();
            Ok(window)
        }
    }
}

pub struct AppBuilder {
    color_bits: u8,
    alpha_bits: u8,
    depth_bits: u8,
    stencil_bits: u8,
    samples: u8,
    srgb: bool,
}

impl AppBuilder {
    pub fn bits(
        &mut self,
        color_bits: u8,
        alpha_bits: u8,
        depth_bits: u8,
        stencil_bits: u8,
    ) -> &mut Self {
        self.color_bits = color_bits;
        self.alpha_bits = alpha_bits;
        self.depth_bits = depth_bits;
        self.stencil_bits = stencil_bits;
        self
    }
    pub fn color_bits(&mut self, bits: u8) -> &mut Self {
        self.color_bits = bits;
        self
    }

    pub fn alpha_bits(&mut self, bits: u8) -> &mut Self {
        self.alpha_bits = bits;
        self
    }

    pub fn depth_bits(&mut self, bits: u8) -> &mut Self {
        self.depth_bits = bits;
        self
    }

    pub fn stencil_bits(&mut self, bits: u8) -> &mut Self {
        self.stencil_bits = bits;
        self
    }

    /// Sets the MSAA samples.
    /// Set this to a power of 2.
    /// With an Nvidia card on Windows I was unable to set this below 2.
    pub fn samples(&mut self, samples: u8) -> &mut Self {
        self.samples = samples;
        self
    }

    /// This sets if the backbuffer for the windows will be in sRGB color space... or it would if drivers respected it.
    /// Unfortunately this flag does nothing as tested on Windows with an Nvidia GPU.
    /// In that case backbuffer was set to sRGB colorspace.
    pub fn srgb(&mut self, srgb: bool) -> &mut Self {
        self.srgb = srgb;
        self
    }

    pub fn build(&self) -> Result<App, Error> {
        unsafe {
            let _pool = NSAutoreleasePool::new(nil);

            let app = NSApp();
            app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

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

            let pixel_format = NSOpenGLPixelFormat::alloc(nil).initWithAttributes_(&attrs);
            let gl_context: id =
                NSOpenGLContext::alloc(nil).initWithFormat_shareContext_(pixel_format, nil);

            gl_context.makeCurrentContext();

            // Enable vsync
            NSOpenGLContext::setValues_forParameter_(
                gl_context,
                &(1 as i32),
                cocoa::appkit::NSOpenGLContextParameter::NSOpenGLCPSwapInterval,
            );

            // Setup window_delegate_class
            let superclass = class!(NSResponder);
            let mut decl = ClassDecl::new("KettlewinWindowClass", superclass).unwrap();
            extern "C" fn window_moved(this: &Object, _sel: Sel, event: id) {
                //  println!("WINDOW MOVED");
                /*
                unsafe {
                    if let Some(data) = TEST_VIEW.as_ref() {
                        let () = msg_send![data.view, setNeedsDisplay: YES];
                    }
                }*/
            }
            extern "C" fn window_did_resize(this: &Object, _sel: Sel, event: id) {
                // TEST_VIEW needs to be replaced with the actual window view.
                unsafe {
                    if let Some(data) = TEST_VIEW.as_ref() {
                        let rect = unsafe { NSView::frame(data.view) };
                        let width = rect.size.width;
                        let height = rect.size.height;
                        produce_event(crate::Event::ResizedWindow {
                            width: width as u32,
                            height: height as u32,
                        });
                        println!("Resized: {:?}, {:?}", width, height);
                    }
                }
            }

            decl.add_method(
                sel!(windowDidMove:),
                window_moved as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(
                sel!(windowDidResize:),
                window_did_resize as extern "C" fn(&Object, Sel, id),
            );

            let window_delegate_class = decl.register();

            // Setup view_delegate_class
            let superclass = class!(NSView);
            let mut decl = ClassDecl::new("KettlewinViewClass", superclass).unwrap();
            extern "C" fn key_down(this: &Object, _sel: Sel, event: id) {
                unsafe {
                    produce_event(crate::Event::ButtonDown {
                        button: super::keys_mac::virtual_keycode_to_key(event.keyCode()),
                        scancode: 0,
                    });
                }
            }

            extern "C" fn key_up(this: &Object, _sel: Sel, event: id) {
                unsafe {
                    produce_event(crate::Event::ButtonUp {
                        button: super::keys_mac::virtual_keycode_to_key(event.keyCode()),
                        scancode: 0,
                    });
                }
            }
            extern "C" fn draw_rect(this: &Object, _sel: Sel, rect: NSRect) {}

            extern "C" fn mouse_moved(this: &Object, _sel: Sel, event: id) {
                unsafe {
                    let window_point = event.locationInWindow();

                    // The following code snippet is taken from winit.
                    // We have to do this to have access to the `NSView` trait...
                    let view: id = this as *const _ as *mut _;

                    let window_point = event.locationInWindow();
                    let view_point = view.convertPoint_fromView_(window_point, nil);
                    let view_rect = NSView::frame(view);

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
                mouse_moved as extern "C" fn(&Object, Sel, id),
            );
            decl.add_method(sel!(keyDown:), key_down as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(keyUp:), key_up as extern "C" fn(&Object, Sel, id));

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
    pub app: id,
    gl_context: id,
    window_delegate_class: *const objc::runtime::Class,
    view_delegate_class: *const objc::runtime::Class,
}

impl App {
    pub fn new() -> AppBuilder {
        AppBuilder {
            color_bits: 32,
            alpha_bits: 8,
            depth_bits: 16,
            stencil_bits: 0,
            samples: 1,
            srgb: true,
        }
    }

    fn setup_opengl() -> Result<(), Error> {
        Ok(())
    }

    pub fn new_window<'a>(&mut self) -> WindowBuilder<'a> {
        WindowBuilder {
            position: None,
            dimensions: None,
            resizable: true,
            title: None,
        }
    }

    pub fn make_current(&self, window: &Window) -> Result<(), Error> {
        unsafe {
            NSOpenGLContext::setView_(self.gl_context, window.view);
        }
        Ok(())
    }

    pub fn swap_buffers(&self) {
        unsafe {
            NSOpenGLContext::flushBuffer(self.gl_context);
        }
    }

    // This belongs to the window builder because the OpenGL context must be constructed first
    // and the window builder creates the context.
    pub fn gl_loader(&self) -> Box<dyn FnMut(&'static str) -> *const std::ffi::c_void> {
        unimplemented!();
    }

    pub fn gl_loader_c_string(&self) -> Box<dyn FnMut(*const i8) -> *const std::ffi::c_void> {
        unsafe {
            Box::new(move |s| {
                unsafe {
                    let name = std::ffi::CStr::from_ptr(s);
                    get_proc_address((&name).to_str().unwrap())
                }
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
            self.app.run();
        }
    }
}

// Taken from Glutin:
// https://github.com/rust-windowing/glutin/blob/447f3526dcf90a460d52afefd0b29eb2ed7f87f3/glutin/src/platform_impl/macos/mod.rs
fn get_proc_address(addr: &str) -> *const core::ffi::c_void {
    let symbol_name: CFString = FromStr::from_str(addr).unwrap();
    let framework_name: CFString = FromStr::from_str("com.apple.opengl").unwrap();
    let framework =
        unsafe { CFBundleGetBundleWithIdentifier(framework_name.as_concrete_TypeRef()) };
    let symbol =
        unsafe { CFBundleGetFunctionPointerForName(framework, symbol_name.as_concrete_TypeRef()) };
    symbol as *const _
}

pub static NSTrackingMouseEnteredAndExited: NSInteger = 0x01;
pub static NSTrackingMouseMoved: NSInteger = 0x02;
pub static NSTrackingActiveInKeyWindow: NSInteger = 0x20;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    pub fn CFRunLoopGetMain() -> CFRunLoopRef;

    pub static kCFRunLoopCommonModes: CFRunLoopMode;
    pub static NSRunLoopCommonModes: id;

    pub fn CFRunLoopObserverCreate(
        allocator: CFAllocatorRef,
        activities: CFOptionFlags,
        repeats: BOOL,
        order: CFIndex,
        callout: CFRunLoopObserverCallBack,
        context: *mut CFRunLoopObserverContext,
    ) -> CFRunLoopObserverRef;
    pub fn CFRunLoopAddObserver(
        rl: CFRunLoopRef,
        observer: CFRunLoopObserverRef,
        mode: CFRunLoopMode,
    );

    pub fn CFRunLoopTimerCreate(
        allocator: CFAllocatorRef,
        fireDate: CFAbsoluteTime,
        interval: CFTimeInterval,
        flags: CFOptionFlags,
        order: CFIndex,
        callout: CFRunLoopTimerCallBack,
        context: *mut CFRunLoopTimerContext,
    ) -> CFRunLoopTimerRef;
    pub fn CFRunLoopAddTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFRunLoopMode);
}

pub enum CFAllocator {}
pub type CFAllocatorRef = *mut CFAllocator;
pub enum CFRunLoop {}
pub type CFRunLoopRef = *mut CFRunLoop;
pub type CFRunLoopMode = CFStringRef;
pub enum CFRunLoopObserver {}
pub type CFRunLoopObserverRef = *mut CFRunLoopObserver;
pub enum CFRunLoopTimer {}
pub type CFRunLoopTimerRef = *mut CFRunLoopTimer;
pub enum CFRunLoopSource {}
pub type CFRunLoopSourceRef = *mut CFRunLoopSource;
pub type CFStringRef = *const CFString;
pub type CFHashCode = std::os::raw::c_ulong;
pub type CFIndex = std::os::raw::c_long;
pub type CFOptionFlags = std::os::raw::c_ulong;
pub type CFRunLoopActivity = CFOptionFlags;

pub type CFAbsoluteTime = CFTimeInterval;
pub type CFTimeInterval = f64;
pub type CFRunLoopObserverCallBack = extern "C" fn(
    observer: CFRunLoopObserverRef,
    activity: CFRunLoopActivity,
    info: *mut std::ffi::c_void,
);
pub type CFRunLoopTimerCallBack =
    extern "C" fn(timer: CFRunLoopTimerRef, info: *mut std::ffi::c_void);

pub enum CFRunLoopObserverContext {}
pub enum CFRunLoopTimerContext {}

#[allow(non_upper_case_globals)]
pub const kCFRunLoopEntry: CFRunLoopActivity = 0;
#[allow(non_upper_case_globals)]
pub const kCFRunLoopBeforeWaiting: CFRunLoopActivity = 1 << 5;
#[allow(non_upper_case_globals)]
pub const kCFRunLoopAfterWaiting: CFRunLoopActivity = 1 << 6;
#[allow(non_upper_case_globals)]
pub const kCFRunLoopExit: CFRunLoopActivity = 1 << 7;
