use std::ffi::{c_char, CString, CStr};


#[no_mangle]
pub extern "C" fn md5(data: *const c_char) -> *mut c_char {
    let cstr = unsafe { CStr::from_ptr(data)};
    let rstr = String::from_utf8_lossy(cstr.to_bytes()).to_string();
    let digest = md5::compute(rstr);
    CString::new(format!("{:x}", digest)).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn hello_world() {
    println!("hello world!")
}

//创建网络
#[no_mangle]
pub extern "C" fn create_vnt(secret: *const c_char,name: *const c_char,server: *const c_char) {
    //todo 创建服务地址
    println!("hello world!")
}

