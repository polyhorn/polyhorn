use std::ffi::CString;
use std::os::raw::{c_char, c_int};

extern "C" {
    fn __android_log_write(level: c_int, tag: *const c_char, message: *const c_char);
}

#[repr(i32)]
pub enum AndroidLogPriority {
    Unknown = 0,
    Default = 1,
    Verbose = 2,
    Debug = 3,
    Info = 4,
    Warn = 5,
    Error = 6,
    Fatal = 7,
    Silent = 8,
}

pub fn android_log_write(level: AndroidLogPriority, tag: &str, message: &str) {
    let tag = CString::new(tag).unwrap();
    let message = CString::new(message).unwrap();

    unsafe {
        __android_log_write(
            level as i32,
            tag.as_c_str().as_ptr(),
            message.as_c_str().as_ptr(),
        );
    }
}
