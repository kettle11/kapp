use std::io::Error;
trait GLContextTrait {
    fn new() -> crate::GLContextBuilder;
    fn set_window(&mut self, window: Option<*mut std::ffi::c_void>) -> Result<(), Error>;
    fn update_target(&self);
    fn make_current(&self);
    fn swap_buffers(&self);

    #[cfg(not(target_arch = "wasm32"))]
    fn gl_loader_c_string(&self) -> Box<dyn FnMut(*const i8) -> *const std::ffi::c_void>;
    #[cfg(not(target_arch = "wasm32"))]
    fn get_proc_address(addr: &str) -> *const core::ffi::c_void;
}
