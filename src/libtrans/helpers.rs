use std::ffi;
#[macro_export]
macro_rules! c_str_ptr {
    ($s:expr) => {
        ffi::CString::new($s).unwrap().as_ptr()
    };
}

#[macro_export]
macro_rules! c_str_mut_ptr {
    ($s:expr) => {
        ffi::CString::new($s).unwrap().into_raw()
    };
}
