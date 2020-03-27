use std::io::Error;
trait GLContextTrait {
    fn new() -> crate::GLContextBuilder;
    fn set_window(&mut self, window: Option<*mut std::ffi::c_void>) -> Result<(), Error>;
    fn update_target(&self);
    fn make_current(&self);
    fn gl_loader_c_string(&self) -> Box<dyn FnMut(*const i8) -> *const std::ffi::c_void>;
    fn swap_buffers(&self);
    fn get_proc_address(addr: &str) -> *const core::ffi::c_void;
}
