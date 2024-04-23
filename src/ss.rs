use std::collections::HashMap;

use nix::sys::epoll::Epoll;
use socket2::Socket;

use crate::log::Log;

mod gate;
mod line;

#[derive(Debug)]
pub enum Tag {
    Dns,
    Pc,
    Hk,
    MainLand,
    World,
}

#[derive(Debug)]
pub enum Status {
    Raw,
    Established,
    FirstPackDone,
    WaitingDnsResult,
    DnsQuerySuccess,
    WorldConnectSuccess,
    SecondPackDone,
    EncryptDone,
    ReadClose,
    WriteClose,
    ReadWriteBothClose,
    DeRegister,
    Close,
    Dead,
}

pub struct Line {
    id:u64,
    pair_id:u64,
    tag:Tag,
    status:Status,
    socket:Socket,
    logger:Log,
    website_host:String,
    website_port:u16,
    client_hello_data:Vec<u8>,
    dns_result:Vec<(u64,Option<String>)>,
    last_recv_heart_beat:i64,
    last_send_heart_beat:i64,
}


pub struct Gate {
    socket:Socket,
    epoll:Epoll,
    next_id:u64,
    lines:HashMap<u64,Line>,
    logger:Log,
}


fn reverse(buf:&mut[u8]) {
    for i in 0..buf.len() {
        buf[i] = u8r(buf[i]);
    }
}


fn u8r(input:u8) -> u8 {
    if input > 45 && input < 255 - 45 {
        255 - input
    } else {
        input
    }
}
