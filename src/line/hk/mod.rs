use std::time::Instant;

use socket2::Socket;

use crate::{config::TCP_LIFE_TIME, global, log::log_dir::LogDir};

use super::{base_line::BaseLine, status::{LineStatus, Status}};

mod heart_beat;
mod trait_status;
mod trait_impl;

pub struct LineHk {
    pub basic:BaseLine,
    pub pair_id:u64,
    last_send_heart_beat:i64,
    last_recv_heart_beat:i64,
    speed:usize,
    clock:Instant,
}

impl LineHk {
    pub fn new(id:u64,socket:Socket) -> LineHk {
        let buf_writer = LineHk::create_buf_writer(id);
        let basic = BaseLine::new(id, socket, buf_writer);
        LineHk { basic, pair_id: 0, last_send_heart_beat: 0, last_recv_heart_beat: 0, 
            speed: 0, clock: Instant::now() }
    }
}

impl LineHk {
    pub fn is_ready(&self) -> bool {
        if self.pair_id > 0 {
            return false;
        }
        
        if self.tcp_timeout() {
            return false;
        }

        if  self.status() == Status::Establish {
            return true;
        }

        false
    }

    pub fn tcp_timeout(&self) -> bool {
        let gap = global::now() - self.last_recv_heart_beat;
        if gap > TCP_LIFE_TIME {
            return true;
        }
        
        if gap < 65536 {
            //log::im(format!("[{}][{}]{:?} {}",self.id,self.pair_id,self.status,gap));
        }

        false
    }
}
