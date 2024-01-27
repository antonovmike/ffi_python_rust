use std::ffi::CString;

#[no_mangle]
pub extern "C" fn get_hello_world() -> *mut std::os::raw::c_char {
    let s = CString::new("Hello world").unwrap();
    s.into_raw()
}