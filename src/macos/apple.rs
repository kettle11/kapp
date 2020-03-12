// This file is a bunch of stuff needed for calling into MacOS code.

use libc::{c_long, c_ulong};
use std::ffi::c_void;
use std::os::raw::c_double;

pub use objc::{
    declare::ClassDecl,
    runtime::{Object, Sel, BOOL, NO, YES},
};

pub const nil: *mut Object = 0 as *mut Object;

pub const NSTrackingMouseEnteredAndExited: NSInteger = 0x01;
pub const NSTrackingMouseMoved: NSInteger = 0x02;
pub const NSTrackingActiveInKeyWindow: NSInteger = 0x20;
pub const NSTrackingInVisibleRect: NSInteger = 0x200;

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSApplicationActivationPolicy {
    NSApplicationActivationPolicyRegular = 0,
    // NSApplicationActivationPolicyAccessory = 1,
    // NSApplicationActivationPolicyProhibited = 2,
    // NSApplicationActivationPolicyERROR = -1,
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    pub fn CFRunLoopGetMain() -> CFRunLoopRef;
    pub fn CFRunLoopWakeUp(rl: CFRunLoopRef);

    pub static kCFRunLoopCommonModes: CFRunLoopMode;
    // pub static NSRunLoopCommonModes: *mut Object;

    pub fn CFRunLoopObserverCreate(
        allocator: CFAllocatorRef,
        activities: CFOptionFlags,
        repeats: BOOL,
        order: CFIndex,
        callout: CFRunLoopObserverCallBack,
        context: *const CFRunLoopObserverContext,
    ) -> CFRunLoopObserverRef;
    pub fn CFRunLoopAddObserver(
        rl: CFRunLoopRef,
        observer: CFRunLoopObserverRef,
        mode: CFRunLoopMode,
    );

    pub fn CFRunLoopSourceCreate(
        allocator: CFAllocatorRef,
        order: CFIndex,
        context: *mut CFRunLoopSourceContext,
    ) -> CFRunLoopSourceRef;
    pub fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFRunLoopMode);
    #[allow(dead_code)]
    pub fn CFRunLoopSourceInvalidate(source: CFRunLoopSourceRef);
    pub fn CFRunLoopSourceSignal(source: CFRunLoopSourceRef);
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct CFRunLoopSourceContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: Option<extern "C" fn(*const c_void) -> *const c_void>,
    pub release: Option<extern "C" fn(*const c_void)>,
    pub copyDescription: Option<extern "C" fn(*const c_void) -> CFStringRef>,
    pub equal: Option<extern "C" fn(*const c_void, *const c_void) -> BOOL>,
    pub hash: Option<extern "C" fn(*const c_void) -> CFHashCode>,
    pub schedule: Option<extern "C" fn(*mut c_void, CFRunLoopRef, CFRunLoopMode)>,
    pub cancel: Option<extern "C" fn(*mut c_void, CFRunLoopRef, CFRunLoopMode)>,
    pub perform: Option<extern "C" fn(*mut c_void)>,
}

pub type CFHashCode = c_ulong;
pub enum CFRunLoopSource {}
pub type CFRunLoopSourceRef = *mut CFRunLoopSource;

pub enum CFAllocator {}
pub type CFAllocatorRef = *mut CFAllocator;
pub enum CFRunLoop {}
pub type CFRunLoopRef = *mut CFRunLoop;
pub type CFRunLoopMode = CFStringRef;
pub enum CFRunLoopObserver {}
pub type CFRunLoopObserverRef = *mut CFRunLoopObserver;

pub type CFStringRef = *const Object; // CFString
pub type CFIndex = std::os::raw::c_long;
pub type CFOptionFlags = std::os::raw::c_ulong;
pub type CFRunLoopActivity = CFOptionFlags;

pub type CFRunLoopObserverCallBack =
    extern "C" fn(observer: CFRunLoopObserverRef, activity: CFRunLoopActivity, info: *mut c_void);

// https://developer.apple.com/documentation/corefoundation/cfrunloopobservercontext?language=objc
#[repr(C)]
pub struct CFRunLoopObserverContext {
    pub copyDescription: *const c_void,
    pub info: *const c_void,
    pub release: *const c_void,
    pub version: CFIndex,
    pub retain: *const c_void,
}

// pub const kCFRunLoopEntry: CFRunLoopActivity = 0;
pub const kCFRunLoopBeforeWaiting: CFRunLoopActivity = 1 << 5;
// pub const kCFRunLoopAfterWaiting: CFRunLoopActivity = 1 << 6;
// pub const kCFRunLoopExit: CFRunLoopActivity = 1 << 7;

// NSWindowStyleMask
// https://developer.apple.com/documentation/appkit/nswindowstylemask?language=objc
// pub const NSWindowStyleMaskBorderless: NSUInteger = 0;
pub const NSWindowStyleMaskTitled: NSUInteger = 1 << 0;
pub const NSWindowStyleMaskClosable: NSUInteger = 1 << 1;
pub const NSWindowStyleMaskMiniaturizable: NSUInteger = 1 << 2;
pub const NSWindowStyleMaskResizable: NSUInteger = 1 << 3;

pub const NSBackingStoreBuffered: NSUInteger = 2;
pub const UTF8_ENCODING: usize = 4;

// These enums are taken from the core-foundation-rs crate
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLContextParameter {
    NSOpenGLCPSwapInterval = 222,
}
pub use NSOpenGLContextParameter::*;

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLPixelFormatAttribute {
    NSOpenGLPFADoubleBuffer = 5,
    NSOpenGLPFAColorSize = 8,

    NSOpenGLPFAAlphaSize = 11,
    NSOpenGLPFADepthSize = 12,
    NSOpenGLPFAStencilSize = 13,
    NSOpenGLPFAAccelerated = 73,
    NSOpenGLPFAOpenGLProfile = 99,
}
pub use NSOpenGLPixelFormatAttribute::*;

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLPFAOpenGLProfiles {
    NSOpenGLProfileVersion3_2Core = 0x3200,
    // NSOpenGLProfileVersion4_1Core = 0x4100,
}
pub use NSOpenGLPFAOpenGLProfiles::*;

#[cfg(target_pointer_width = "32")]
pub type NSInteger = c_int;
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = c_uint;

#[cfg(target_pointer_width = "64")]
pub type NSInteger = c_long;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = c_ulong;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

impl CGPoint {
    pub fn new(x: CGFloat, y: CGFloat) -> Self {
        Self { x, y }
    }
}

pub type NSPoint = CGPoint;

pub type CGFloat = c_double;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

impl CGSize {
    pub fn new(width: CGFloat, height: CGFloat) -> Self {
        Self { width, height }
    }
}

pub type NSSize = CGSize;

impl CGRect {
    pub fn new(origin: CGPoint, size: CGSize) -> Self {
        Self { origin, size }
    }
}

unsafe impl objc::Encode for CGRect {
    fn encode() -> objc::Encoding {
        let encoding = format!(
            "{{CGRect={}{}}}",
            NSPoint::encode().as_str(),
            NSSize::encode().as_str()
        );
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

unsafe impl objc::Encode for CGPoint {
    fn encode() -> objc::Encoding {
        let encoding = format!(
            "{{CGPoint={}{}}}",
            CGFloat::encode().as_str(),
            CGFloat::encode().as_str()
        );
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

unsafe impl objc::Encode for CGSize {
    fn encode() -> objc::Encoding {
        let encoding = format!(
            "{{CGSize={}{}}}",
            CGFloat::encode().as_str(),
            CGFloat::encode().as_str()
        );
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

pub type NSRect = CGRect;

#[repr(C)]
pub struct __CFBundle(c_void);
pub type CFBundleRef = *mut __CFBundle;

extern "C" {
    pub fn CFBundleGetBundleWithIdentifier(bundleID: CFStringRef) -> CFBundleRef;
    pub fn CFBundleGetFunctionPointerForName(
        bundle: CFBundleRef,
        function_name: CFStringRef,
    ) -> *const c_void;
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

pub const NX_DEVICELSHIFTKEYMASK: u64 = 0x2;
pub const NX_DEVICERSHIFTKEYMASK: u64 = 0x4;

pub const NX_DEVICELCTLKEYMASK: u64 = 0x1;
pub const NX_DEVICERCTLKEYMASK: u64 = 0x2000;

pub const NX_DEVICELALTKEYMASK: u64 = 0x20;
pub const NX_DEVICERALTKEYMASK: u64 = 0x40;

pub const NX_DEVICELCMDKEYMASK: u64 = 0x8;
pub const NX_DEVICERCMDKEYMASK: u64 = 0x10;
