#[no_mangle]
pub extern "C" fn print_text(text: *const std::os::raw::c_char) {
    // Convert a pointer to a C string to a Rust string
    let c_str = unsafe { std::ffi::CStr::from_ptr(text) };
    // Try to convert Rust string to UTF-8
    let result = c_str.to_str();
    // If conversion is successful, output the string to the terminal
    if let Ok(string) = result {
        println!("{}", string);
    }
}
