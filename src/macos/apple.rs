use libc::{c_long, c_ulong};
use std::os::raw::c_double;

pub use objc::{
    declare::ClassDecl,
    runtime::{Object, Sel, BOOL, NO, YES},
};

pub static NSTrackingMouseEnteredAndExited: NSInteger = 0x01;
pub static NSTrackingMouseMoved: NSInteger = 0x02;
pub static NSTrackingActiveInKeyWindow: NSInteger = 0x20;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    pub fn CFRunLoopGetMain() -> CFRunLoopRef;

    pub static kCFRunLoopCommonModes: CFRunLoopMode;
    pub static NSRunLoopCommonModes: *mut Object;

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
pub type CFStringRef = *const Object; // CFString
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

pub const kCFRunLoopEntry: CFRunLoopActivity = 0;
pub const kCFRunLoopBeforeWaiting: CFRunLoopActivity = 1 << 5;
pub const kCFRunLoopAfterWaiting: CFRunLoopActivity = 1 << 6;
pub const kCFRunLoopExit: CFRunLoopActivity = 1 << 7;

// NSWindowStyleMask
// https://developer.apple.com/documentation/appkit/nswindowstylemask?language=objc
pub const NSWindowStyleMaskBorderless: NSUInteger = 0;
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
    NSOpenGLCPSurfaceOrder = 235,
    NSOpenGLCPSurfaceOpacity = 236,
    NSOpenGLCPSurfaceBackingSize = 304,
    NSOpenGLCPReclaimResources = 308,
    NSOpenGLCPCurrentRendererID = 309,
    NSOpenGLCPGPUVertexProcessing = 310,
    NSOpenGLCPGPUFragmentProcessing = 311,
    NSOpenGLCPHasDrawable = 314,
    NSOpenGLCPMPSwapsInFlight = 315,
}
pub use NSOpenGLContextParameter::*;

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSApplicationActivationPolicy {
    NSApplicationActivationPolicyRegular = 0,
    NSApplicationActivationPolicyAccessory = 1,
    NSApplicationActivationPolicyProhibited = 2,
    NSApplicationActivationPolicyERROR = -1,
}

use NSApplicationActivationPolicy::*;

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLPixelFormatAttribute {
    NSOpenGLPFAAllRenderers = 1,
    NSOpenGLPFATripleBuffer = 3,
    NSOpenGLPFADoubleBuffer = 5,
    NSOpenGLPFAStereo = 6,
    NSOpenGLPFAAuxBuffers = 7,
    NSOpenGLPFAColorSize = 8,
    NSOpenGLPFAAlphaSize = 11,
    NSOpenGLPFADepthSize = 12,
    NSOpenGLPFAStencilSize = 13,
    NSOpenGLPFAAccumSize = 14,
    NSOpenGLPFAMinimumPolicy = 51,
    NSOpenGLPFAMaximumPolicy = 52,
    NSOpenGLPFAOffScreen = 53,
    NSOpenGLPFAFullScreen = 54,
    NSOpenGLPFASampleBuffers = 55,
    NSOpenGLPFASamples = 56,
    NSOpenGLPFAAuxDepthStencil = 57,
    NSOpenGLPFAColorFloat = 58,
    NSOpenGLPFAMultisample = 59,
    NSOpenGLPFASupersample = 60,
    NSOpenGLPFASampleAlpha = 61,
    NSOpenGLPFARendererID = 70,
    NSOpenGLPFASingleRenderer = 71,
    NSOpenGLPFANoRecovery = 72,
    NSOpenGLPFAAccelerated = 73,
    NSOpenGLPFAClosestPolicy = 74,
    NSOpenGLPFARobust = 75,
    NSOpenGLPFABackingStore = 76,
    NSOpenGLPFAMPSafe = 78,
    NSOpenGLPFAWindow = 80,
    NSOpenGLPFAMultiScreen = 81,
    NSOpenGLPFACompliant = 83,
    NSOpenGLPFAScreenMask = 84,
    NSOpenGLPFAPixelBuffer = 90,
    NSOpenGLPFARemotePixelBuffer = 91,
    NSOpenGLPFAAllowOfflineRenderers = 96,
    NSOpenGLPFAAcceleratedCompute = 97,
    NSOpenGLPFAOpenGLProfile = 99,
    NSOpenGLPFAVirtualScreenCount = 128,
}
pub use NSOpenGLPixelFormatAttribute::*;

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLPFAOpenGLProfiles {
    NSOpenGLProfileVersionLegacy = 0x1000,
    NSOpenGLProfileVersion3_2Core = 0x3200,
    NSOpenGLProfileVersion4_1Core = 0x4100,
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
pub struct __CFBundle(std::ffi::c_void);
pub type CFBundleRef = *mut __CFBundle;

extern "C" {
    pub fn CFBundleGetBundleWithIdentifier(bundleID: CFStringRef) -> CFBundleRef;
    pub fn CFBundleGetFunctionPointerForName(
        bundle: CFBundleRef,
        function_name: CFStringRef,
    ) -> *const std::ffi::c_void;
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
