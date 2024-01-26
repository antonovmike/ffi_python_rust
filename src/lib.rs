use std::{ffi::CStr, str::Utf8Error};

#[no_mangle]
pub extern "C" fn print_text(text: *const std::os::raw::c_char) {
    // Convert a pointer to a C string to a borrowed C string
    let c_str: &CStr = unsafe { std::ffi::CStr::from_ptr(text) };
    // Try to convert the borrowed C string string to UTF-8
    let result: Result<&str, Utf8Error> = c_str.to_str();

    if let Ok(string) = result {
        println!("{}", string);
    }
}
