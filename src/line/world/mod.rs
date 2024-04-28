
use socket2::Socket;

use crate::log::{log_dir::LogDir, Log};

use super::base_line::BaseLine;

mod trait_impl;

pub struct LineWorld {
    pub basic:BaseLine,
    pub pair_id:u64,
    pub peer_address:String,
}

impl LineWorld {
    pub fn new(id:u64,pair_id:u64,socket:Socket) -> LineWorld {
        let buf_writer = LineWorld::create_buf_writer(id);
        let basic = BaseLine::new(id, socket, buf_writer);
        LineWorld { basic, pair_id, peer_address: String::new() }
    }
}

impl LineWorld {
    pub fn set_peer_address(&mut self,s:String) {
        self.log(format!("set_peer_address {}",s));
        self.peer_address = s;
    }
}


