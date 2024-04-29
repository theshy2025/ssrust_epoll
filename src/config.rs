use std::sync::OnceLock;

use simple_config_parser::Config;

pub const BUFF_SIZE:usize = 1024*64;
pub const ATYP_INDEX:usize = 3;
pub const ATYP_HOST_NAME:u8 = 3;

pub const GATE_ID:u64 = 1;
pub const DNS_ID:u64 = 2;

pub const TCP_LIFE_TIME:i64 = 30;


pub static DEVICE:OnceLock<String> = OnceLock::new();
pub static REMOTE_ADDRESS:OnceLock<String> = OnceLock::new();
pub static CHICK_INIT_NUM:OnceLock<u8> = OnceLock::new();
pub static GATE_PORT:OnceLock<u16> = OnceLock::new();

pub fn device() -> String {
    let val = DEVICE.get_or_init(|| {
        get_cfg_str("device")
    });

    (*val).clone()
}

pub fn remote_address() -> String {
    let val = REMOTE_ADDRESS.get_or_init(|| {
        get_cfg_str("remote_address")
    });

    (*val).clone()
}

pub fn gate_port() -> u16 {
    let val = GATE_PORT.get_or_init(|| {
        let f = Config::new().file("custom.cfg").unwrap();
        f.get("gate_port").unwrap()
    });
    
    *val
}

pub fn chick_init_num() -> u8 {
    let val = CHICK_INIT_NUM.get_or_init(|| {
        let f = Config::new().file("custom.cfg").unwrap();
        f.get("chick_init_num").unwrap()
    });
    
    *val
}


fn get_cfg_str(key:&str) -> String {
    let f = Config::new().file("custom.cfg").unwrap();
    f.get_str(key).unwrap()
}