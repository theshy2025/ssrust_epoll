use std::time::Instant;

use socket2::Socket;


use crate::log::{log_dir::LogDir, Log};

use self::network::Step;

use super::base_line::BaseLine;

mod trait_impl;
mod heart_beat;
pub mod network;

pub struct LineMainLand {
    pub basic:BaseLine,
    pub pair_id:u64,
    pub step:Step,
    pub peer_ip:String,
    pub peer_port:u16,
    client_hello_data:Vec<u8>,
    speed:usize,
    clock:Instant,
}

impl LineMainLand {
    pub fn new(id:u64,socket:Socket) -> LineMainLand {
        let buf_writer = LineMainLand::create_buf_writer(id);
        let basic = BaseLine::new(id, socket, buf_writer);
        LineMainLand { basic, pair_id : 0, peer_ip: String::new(), peer_port: 0 , 
            client_hello_data: Vec::new(), step: Step::Raw,
            speed: 0,
            clock: Instant::now(), }
    }
}

impl LineMainLand {
    pub fn send_done(&mut self,n:usize) {
        let t = self.clock.elapsed().as_millis();
        if t > 1000 {
            self.clock = Instant::now();
            self.speed = 0;
        }
        self.speed = self.speed + n;
        self.log(format!("send to mainland {} bytes [{}]ms[{}]k",n,t,self.speed/1024));
    }
}

