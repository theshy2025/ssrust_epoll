use std::sync::OnceLock;

use simple_config_parser::Config;

pub const BUFF_SIZE:usize = 1024*64;
pub const ATYP_INDEX:usize = 3;
pub const ATYP_HOST_NAME:u8 = 3;

pub const TCP_GATE_ID:u64 = 1;
pub const UDP_GATE_ID:u64 = 2;
pub const DNS_ID:u64 = 3;

pub const TCP_LIFE_TIME:i64 = 30;


pub static DEVICE:OnceLock<String> = OnceLock::new();

pub static TCP_2_VPS_LINE_INIT_NUM:OnceLock<u8> = OnceLock::new();

pub static DNS_SERVER:OnceLock<String> = OnceLock::new();

pub static TCP_PORT:OnceLock<u16> = OnceLock::new();
pub static UDP_PORT:OnceLock<u16> = OnceLock::new();


pub static VPS_IP:OnceLock<String> = OnceLock::new();
pub static VPS_TCP_PORT:OnceLock<u16> = OnceLock::new();
pub static VPS_UDP_PORT:OnceLock<u16> = OnceLock::new();


pub fn device() -> String {
    let val = DEVICE.get_or_init(|| {
        get_cfg_str("device")
    });

    (*val).clone()
}

pub fn dns_server() -> String {
    let val = DNS_SERVER.get_or_init(|| {
        get_cfg_str("dns_server")
    });

    (*val).clone()
}

pub fn vps_ip() -> String {
    let val = VPS_IP.get_or_init(|| {
        get_cfg_str("vps_ip")
    });

    (*val).clone()
}

pub fn tcp_port() -> u16 {
    let val = TCP_PORT.get_or_init(|| {
        let f = Config::new().file("custom.cfg").unwrap();
        f.get("tcp_port").unwrap()
    });
    
    *val
}

pub fn udp_port() -> u16 {
    let val = UDP_PORT.get_or_init(|| {
        let f = Config::new().file("custom.cfg").unwrap();
        f.get("udp_port").unwrap()
    });
    
    *val
}

pub fn vps_tcp_port() -> u16 {
    let val = VPS_TCP_PORT.get_or_init(|| {
        let f = Config::new().file("custom.cfg").unwrap();
        f.get("vps_tcp_port").unwrap()
    });
    
    *val
}

pub fn vps_udp_port() -> u16 {
    let val = VPS_UDP_PORT.get_or_init(|| {
        let f = Config::new().file("custom.cfg").unwrap();
        f.get("vps_udp_port").unwrap()
    });
    
    *val
}

pub fn tcp_2_vps_line_init_num() -> u8 {
    let val = TCP_2_VPS_LINE_INIT_NUM.get_or_init(|| {
        let f = Config::new().file("custom.cfg").unwrap();
        f.get("tcp_2_vps_line_init_num").unwrap()
    });
    
    *val
}


fn get_cfg_str(key:&str) -> String {
    let f = Config::new().file("custom.cfg").unwrap();
    f.get_str(key).unwrap()
}