use std::os::raw::{c_char, c_double, c_int};

pub const nil: *mut Object = 0 as *mut Object;
pub const UTF8_ENCODING: usize = 4;

pub use objc::{
    declare::ClassDecl,
    runtime::{Object, Protocol, Sel, BOOL, NO, YES},
};

#[link(name = "AppKit", kind = "framework")]
extern "C" {
    pub fn UIApplicationMain(
        argc: c_int,
        argv: *mut *const c_char,
        principalClassName: *mut Object,
        delegateClassName: *mut Object,
    ) -> c_int;
}

pub struct NSString {
    pub raw: *mut Object,
}

impl NSString {
    pub fn new(string: &str) -> Self {
        unsafe {
            let raw: *mut Object = msg_send![class!(NSString), alloc];
            let raw: *mut Object = msg_send![
                raw,
                initWithBytes: string.as_ptr()
                length: string.len()
                encoding:UTF8_ENCODING as *mut Object
            ];

            Self { raw }
        }
    }
}

impl Drop for NSString {
    fn drop(&mut self) {
        unsafe {
            let () = msg_send![self.raw, release];
        }
    }
}

pub type CGFloat = c_double;

#[repr(C)]
#[derive(Clone)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

impl CGPoint {
    pub fn new(x: CGFloat, y: CGFloat) -> Self {
        Self { x, y }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

impl CGSize {
    pub fn new(width: CGFloat, height: CGFloat) -> Self {
        Self { width, height }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}
