use std::io::Error;
extern crate cocoa;
extern crate core_foundation;
extern crate objc;
use cocoa::appkit::*;
use cocoa::base::{nil, NO};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString};
use core_foundation::base::TCFType;
use core_foundation::bundle::{CFBundleGetBundleWithIdentifier, CFBundleGetFunctionPointerForName};
use core_foundation::string::CFString;

use std::str::FromStr;

pub struct Window {}

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

    pub fn build(&self) -> Result<Window, Error> {
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
            Ok(Window {})
        }
    }
}

pub struct WindowManagerBuilder {
    color_bits: u8,
    alpha_bits: u8,
    depth_bits: u8,
    stencil_bits: u8,
    samples: u8,
    srgb: bool,
}

impl WindowManagerBuilder {
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

    pub fn build(&self) -> Result<WindowManager, Error> {
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
            let gl_context =
                NSOpenGLContext::alloc(nil).initWithFormat_shareContext_(pixel_format, nil);

            gl_context.makeCurrentContext();
            Ok(WindowManager { app, gl_context })
        }
    }
}

pub struct WindowManager {
    pub app: *mut objc::runtime::Object,
    gl_context: *mut objc::runtime::Object,
}

impl WindowManager {
    pub fn new() -> WindowManagerBuilder {
        WindowManagerBuilder {
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
        unimplemented!()
    }

    pub fn swap_buffers(&self, window: &Window) {
        unimplemented!();
    }

    // This belongs to the window builder because the OpenGL context must be constructed first
    // and the window builder creates the context.
    pub fn gl_loader(&self) -> Box<dyn FnMut(&'static str) -> *const std::ffi::c_void> {
        unimplemented!();
    }

    #[cfg(feature = "opengl_glow")]
    pub fn gl_context(&self) -> glow::Context {
        unsafe { glow::Context::from_loader_function(|s| get_proc_address(s)) }
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
