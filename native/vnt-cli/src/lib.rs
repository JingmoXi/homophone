use std::io;
use std::net::{Ipv4Addr, ToSocketAddrs};
use std::path::PathBuf;
use std::str::FromStr;

use console::style;
use getopts::Options;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::signal;
#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};

use common::args_parse::{ips_parse, out_ips_parse};
use vnt::cipher::CipherModel;
use vnt::core::{Config, Vnt, VntUtil};
use vnt::handle::handshake_handler::HandshakeEnum;
use vnt::handle::registration_handler::ReqEnum;

mod command;
mod console_out;
mod root_check;
mod cmd;

pub fn app_home() -> io::Result<PathBuf> {
    let path = dirs::home_dir().ok_or(io::Error::new(io::ErrorKind::Other, "not home"))?.join(".vnt-cli");
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }
    Ok(path)
}


pub async fn main0(config: Config, show_cmd: bool) {
    let server_encrypt = config.server_encrypt;
    let mut vnt_util = VntUtil::new(config).await.unwrap();
    let mut conn_count = 0;
    let response = loop {
        if conn_count > 0 {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
        conn_count += 1;
        if let Err(e) = vnt_util.connect().await {
            println!("connect server.dart failed {}", e);
            return;
        }
        match vnt_util.handshake().await {
            Ok(response) => {
                if server_encrypt {
                    let finger = response.unwrap().finger().unwrap();
                    println!("{}{}", green("server.dart fingerprint:".to_string()), finger);
                    match vnt_util.secret_handshake().await {
                        Ok(_) => {}
                        Err(e) => {
                            match e {
                                HandshakeEnum::NotSecret => {}
                                HandshakeEnum::KeyError => {}
                                HandshakeEnum::Timeout => {
                                    println!("handshake timeout")
                                }
                                HandshakeEnum::ServerError(str) => {
                                    println!("error:{}", str);
                                }
                                HandshakeEnum::Other(str) => {
                                    println!("error:{}", str);
                                }
                            }
                            continue;
                        }
                    }
                }
                match vnt_util.register().await {
                    Ok(response) => {
                        break response;
                    }
                    Err(e) => {
                        match e {
                            ReqEnum::TokenError => {
                                println!("token error");
                                return;
                            }
                            ReqEnum::AddressExhausted => {
                                println!("address exhausted");
                                return;
                            }
                            ReqEnum::Timeout => {
                                println!("timeout...");
                            }
                            ReqEnum::ServerError(str) => {
                                println!("error:{}", str);
                            }
                            ReqEnum::Other(str) => {
                                println!("error:{}", str);
                            }
                            ReqEnum::IpAlreadyExists => {
                                println!("ip already exists");
                                return;
                            }
                            ReqEnum::InvalidIp => {
                                println!("invalid ip");
                                return;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                match e {
                    HandshakeEnum::NotSecret => {
                        println!("The server.dart does not support encryption");
                        return;
                    }
                    HandshakeEnum::KeyError => {}
                    HandshakeEnum::Timeout => {
                        println!("handshake timeout")
                    }
                    HandshakeEnum::ServerError(str) => {
                        println!("error:{}", str);
                    }
                    HandshakeEnum::Other(str) => {
                        println!("error:{}", str);
                    }
                }
            }
        }
    };
    println!(" ====== Connect Successfully ====== ");
    println!("virtual_gateway:{}", response.virtual_gateway);
    println!("virtual_ip:{}", green(response.virtual_ip.to_string()));
    let driver_info = vnt_util.create_iface().unwrap();
    println!(" ====== Create Network Interface Successfully ====== ");
    println!("name:{}", driver_info.name);
    println!("version:{}", driver_info.version);
    let mut vnt = match vnt_util.build().await {
        Ok(vnt) => {
            vnt
        }
        Err(e) => {
            println!("error:{}", e);
            return;
        }
    };
    println!(" ====== Start Successfully ====== ");
    let vnt_c = vnt.clone();
    tokio::spawn(async {
        if let Err(e) = command::server::CommandServer::new().start(vnt_c).await {
            println!("command error :{}", e);
        }
    });
    if show_cmd {
        let stdin = tokio::io::stdin();
        let mut cmd = String::new();
        let mut reader = BufReader::new(stdin);
        #[cfg(unix)]
            let mut sigterm = signal(SignalKind::terminate()).expect("Error setting SIGTERM handler");
        loop {
            cmd.clear();
            println!("input:list,info,route,all,stop");
            #[cfg(unix)]
            tokio::select! {
                _ = vnt.wait_stop()=>{
                    return;
                }
                _ = signal::ctrl_c()=>{
                    let _ = vnt.stop();
                    vnt.wait_stop_ms(std::time::Duration::from_secs(3)).await;
                    return;
                }
                _ = sigterm.recv()=>{
                    let _ = vnt.stop();
                    vnt.wait_stop_ms(std::time::Duration::from_secs(3)).await;
                    return;
                }
                rs = reader.read_line(&mut cmd)=>{
                     match rs {
                        Ok(len) => {
                            if !command(&cmd[..len],&vnt){
                                break;
                            }
                        }
                        Err(e) => {
                            println!("input err:{}",e);
                            break;
                        }
                    }
                }
            }
            #[cfg(windows)]
            tokio::select! {
                _ = vnt.wait_stop()=>{
                    return;
                }
                _ = signal::ctrl_c()=>{
                    let _ = vnt.stop();
                    vnt.wait_stop_ms(std::time::Duration::from_secs(3)).await;
                    return;
                }
                rs = reader.read_line(&mut cmd)=>{
                     match rs {
                        Ok(len) => {
                            if !command(&cmd[..len],&vnt){
                                break;
                            }
                        }
                        Err(e) => {
                            println!("input err:{}",e);
                            break;
                        }
                    }
                }
            }
        }
        #[cfg(unix)]
        tokio::select! {
                _ = vnt.wait_stop()=>{
                    return;
                }
                _ = signal::ctrl_c()=>{
                    let _ = vnt.stop();
                    vnt.wait_stop_ms(std::time::Duration::from_secs(3)).await;
                    return;
                }
                _ = sigterm.recv()=>{
                    let _ = vnt.stop();
                    vnt.wait_stop_ms(std::time::Duration::from_secs(3)).await;
                    return;
                }
       }
    }
    vnt.wait_stop().await;
}

fn command(cmd: &str, vnt: &Vnt) -> bool {
    if cmd.is_empty() {
        return false;
    }
    match cmd.to_lowercase().trim() {
        "list" => {
            let list = command::command_list(&vnt);
            console_out::console_device_list(list);
        }
        "info" => {
            let info = command::command_info(&vnt);
            console_out::console_info(info);
        }
        "route" => {
            let route = command::command_route(&vnt);
            console_out::console_route_table(route);
        }
        "all" => {
            let list = command::command_list(&vnt);
            console_out::console_device_list_all(list);
        }
        "stop" => {
            let _ = vnt.stop();
            return false;
        }
        _ => {}
    }
    println!();
    return true;
}

fn print_usage(program: &str, _opts: Options) {
    println!("Usage: {} [options]", program);
    println!("version:{}",vnt::VNT_VERSION);
    println!("Options:");
    println!("  -k <token>          {}", green("必选,使用相同的token,就能组建一个局域网络".to_string()));
    println!("  -n <name>           给设备一个名字,便于区分不同设备,默认使用系统版本");
    println!("  -d <id>             设备唯一标识符,不使用--ip参数时,服务端凭此参数分配虚拟ip");
    println!("  -c                  关闭交互式命令,使用此参数禁用控制台输入");
    println!("  -s <server.dart>         注册和中继服务器地址");
    println!("  -e <stun-server.dart>    stun服务器,用于探测NAT类型,可多次指定,如-e addr1 -e addr2");
    println!("  -a                  使用tap模式,默认使用tun模式");
    println!("  -i <in-ip>          配置点对网(IP代理)时使用,-i 192.168.0.0/24,10.26.0.3表示允许接收网段192.168.0.0/24的数据");
    println!("                      并转发到10.26.0.3,可指定多个网段");
    println!("  -o <out-ip>         配置点对网时使用,-o 192.168.0.0/24表示允许将数据转发到192.168.0.0/24,可指定多个网段");
    println!("  -w <password>       使用该密码生成的密钥对客户端数据进行加密,并且服务端无法解密,使用相同密码的客户端才能通信");
    println!("  -W                  加密当前客户端和服务端通信的数据,请留意服务端指纹是否正确");
    println!("  -m                  模拟组播,默认情况下组播数据会被当作广播发送,开启后会模拟真实组播的数据发送");
    println!("  -u <mtu>            自定义mtu(不加密默认为1430，加密默认为1410)");
    println!("  --tcp               和服务端使用tcp通信,默认使用udp,遇到udp qos时可指定使用tcp");
    println!("  --ip <ip>           指定虚拟ip,指定的ip不能和其他设备重复,必须有效并且在服务端所属网段下,默认情况由服务端分配");
    println!("  --relay             仅使用服务器转发,不使用p2p,默认情况允许使用p2p");
    println!("  --par <parallel>    任务并行度(必须为正整数),默认值为1");
    println!("  --thread <thread>   线程数(必须为正整数),默认为核心数乘2");
    println!("  --model <model>     加密模式，可选值 aes_gcm/aes_cbc，默认使用aes_gcm，通常情况使用aes_cbc性能更好");
    println!();
    println!("  --list              {}", yellow("后台运行时,查看其他设备列表".to_string()));
    println!("  --all               {}", yellow("后台运行时,查看其他设备完整信息".to_string()));
    println!("  --info              {}", yellow("后台运行时,查看当前设备信息".to_string()));
    println!("  --route             {}", yellow("后台运行时,查看数据转发路径".to_string()));
    println!("  --stop              {}", yellow("停止后台运行".to_string()));
    println!("  -h, --help          帮助");
}

fn green(str: String) -> impl std::fmt::Display {
    style(str).green()
}

fn yellow(str: String) -> impl std::fmt::Display {
    style(str).yellow()
}


