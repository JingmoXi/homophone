use std::ffi::{c_char, CString, CStr};
use std::net::{Ipv4Addr, ToSocketAddrs};
use std::str::FromStr;
use common::args_parse;
use vnt::cipher::CipherModel;


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

//进入网络 //     create_vnt(CString::new("123 ".to_owned()).unwrap().into_raw(),CString::new("wzj ".to_owned()).unwrap().into_raw(),CString::new("nat1.wherewego.top:29872".to_owned()).unwrap().into_raw());
#[no_mangle]
pub extern "C" fn create_vnt(secret: *const c_char,name: *const c_char,server: *const c_char) {
    //todo 创建服务地址
    println!("start create_vnt!");
    //
    let server = unsafe { CStr::from_ptr(server)};
    let server = String::from_utf8_lossy(server.to_bytes()).to_string();
    let secret = unsafe { CStr::from_ptr(secret)};
    let secret = String::from_utf8_lossy(secret.to_bytes()).to_string();
    let name = unsafe { CStr::from_ptr(name)};
    let name = String::from_utf8_lossy(name.to_bytes()).to_string();
    println!("start create_vnt! 1");

    let server_address = match server.to_socket_addrs() {
        Ok(mut addr) => {
            if let Some(addr) = addr.next() {
                addr
            } else {
                println!("parameter -s error .");
                return;
            }
        }
        Err(e) => {
            // println!("parameter -s error {}.", e);
            return;
        }
    };
    println!("start create_vnt! 2");
    let stun_server = vec!["stun1.l.google.com:19302".to_string(),"stun2.l.google.com:19302".to_string(),"stun.qq.com:3478".to_string()];
    println!("start create_vnt! 3");
    let in_ip = vec![];
    let in_ip = match common::args_parse::ips_parse(&in_ip) {
        Ok(in_ip) => { in_ip }
        Err(e) => {
            println!();
            // println!("-i {}", e);
            println!("example: -i 192.168.0.0/24,10.26.0.3");
            return;
        }
    };
    println!("start create_vnt! 4");
    let out_ip =  vec![];
    let out_ip = match args_parse::out_ips_parse(&out_ip) {
        Ok(out_ip) => { out_ip }
        Err(e) => {
            println!();
            // println!("-o {}", e);
            println!("example: -o 0.0.0.0/0");
            return;
        }
    };
    let password: Option<String> = Some("".to_string());
    let mtu=Some(1460);
    let virtual_ip: Option<String> = None;
    let virtual_ip = virtual_ip.map(|v| Ipv4Addr::from_str(&v).expect("--ip error"));
    println!("start create_vnt! 5");
    let cipher_model=CipherModel::AesGcm;
    let thread_num= std::thread::available_parallelism().unwrap().get() * 2;
    let config =  vnt::core::Config::new(true,
                                      secret.to_string(), "13".to_string(), name.to_string(), server_address
                                      , server.to_string(),
                                      stun_server, in_ip,
                                      out_ip, password, false, mtu,
                                      true, virtual_ip, true, true, 1, cipher_model);
    let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().worker_threads(thread_num).build().unwrap();
    println!("start create_vnt! 6");
    runtime.block_on(vnt_cli::main0(config, true));
    println!("end create_vnt!");
    std::process::exit(0);
}

//todo 查询网络服务列表
#[no_mangle]
pub extern "C" fn server_list()-> *mut c_char {
    CString::new(String::from("hello") ).unwrap().into_raw()
}

//todo 退出网络
#[no_mangle]
pub extern "C" fn stop() {
    println!("hello world!")
}

//

//todo 查询本机ip地址
#[no_mangle]
pub extern "C" fn local_ip()-> *mut c_char {
    CString::new(String::from("hello") ).unwrap().into_raw()
}

