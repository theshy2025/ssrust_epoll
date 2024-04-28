use chrono::Local;

static mut FRAME: i32 = 0;

static mut ID: u64 = 10;

pub fn frame() -> i32 {
    unsafe { FRAME }
}

pub fn next_frame() {
    unsafe { FRAME = FRAME + 1 };
}

pub fn next_id() -> u64 {
    unsafe { 
        ID = ID + 1;
        return ID; 
    };
}

pub fn u8r(input:u8) -> u8 {
    if input > 45 && input < 255 - 45 {
        255 - input
    } else {
        input
    }
}

pub fn reverse(buf:&mut[u8]) {
    for i in 0..buf.len() {
        buf[i] = u8r(buf[i]);
    }
}

pub fn now() -> i64 {
    let now = Local::now();
    now.timestamp()
}

/*
use std::{collections::HashMap, time::Instant};

use nix::sys::epoll::Epoll;
use socket2::Socket;

use crate::log::Log;




/* 
#[derive(Debug,PartialEq, Eq)]
pub enum Tag {
    Dns,
    Pc,
    Hk,
    MainLand,
    World,
}*/





    
    peer_ip:String,
    peer_port:u16,
    client_hello_data:Vec<u8>,
    dns_result:Vec<(u64,Option<String>)>,
    last_recv_heart_beat:i64,
    last_send_heart_beat:i64,
    clock:Instant,
    traffic:usize,
}


pub trait NetWork {
    
}











*/