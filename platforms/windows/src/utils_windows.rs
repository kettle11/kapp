use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::prelude::*;

/*
use std::io::Error;

pub fn error_if_false(i: i32, panic_if_fail: bool) -> Result<(), Error> {
    if i == 0 {
        if panic_if_fail {
            Err(Error::last_os_error()).unwrap()
        } else {
            Err(Error::last_os_error())
        }
    } else {
        Ok(())
    }
}

pub fn error_if_null<T>(pointer: *const T, panic_if_fail: bool) -> Result<(), Error> {
    if pointer.is_null() {
        if panic_if_fail {
            Err(Error::last_os_error()).unwrap()
        } else {
            Err(Error::last_os_error())
        }
    } else {
        Ok(())
    }
}
*/

pub fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}
