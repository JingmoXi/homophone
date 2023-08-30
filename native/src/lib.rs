use std::ffi::{c_char, CString, CStr};

#[no_mangle]
pub extern "C" fn md5(data: *const c_char) -> *mut c_char {
    let cstr = unsafe { CStr::from_ptr(data)};
    let rstr = String::from_utf8_lossy(cstr.to_bytes()).to_string();
    let digest = md5::compute(rstr);
    CString::new(format!("{:x}", digest)).unwrap().into_raw()
}

