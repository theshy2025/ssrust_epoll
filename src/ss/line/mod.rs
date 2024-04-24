use std::time::Instant;

use socket2::Socket;

use crate::log::{self, Log};

use super::{Line, Status, Tag};

mod dns;
mod pc;
mod hk;
mod mainland;
mod world;
mod status;
mod pair;
mod data;
mod tcp_udp;

impl Line {
    pub fn new(id:u64,pair_id:u64,tag:Tag,socket:Socket) -> Line {
        let logger = Log::create_for_line(&tag, id);
        Line{ id , tag , socket, status: Status::Raw, 
            logger, pair_id,
            peer_ip: String::new(), peer_port: 0,
            client_hello_data: Vec::new(),
            dns_result: Vec::new(),
            last_recv_heart_beat: 0,
            last_send_heart_beat: 0,
            clock: Instant::now(),
            traffic: 0,
        }
    }
}

impl Line {

    

    fn on_network_data(&mut self,buf:&mut [u8]) -> usize {
        match self.tag {
            Tag::Pc => self.on_data_from_pc(buf),
            Tag::Hk => self.on_data_from_hk(buf),
            Tag::MainLand => self.on_data_from_mainland(buf),
            Tag::World => self.on_data_from_world(buf),
            Tag::Dns => self.on_data_from_dns_server(buf),
        }
    }

    
}

impl Line {
    fn log(&mut self,s:String) {
        let s = format!("[{}]{}",self.pair_id,s);
        self.logger.add(s);
        self.logger.flush();
    }

    fn err(&mut self,s:String) {
        let s = format!("[{}][{}]{}",self.id,self.pair_id,s);
        self.log(s.clone());
        log::im(s);
    }
}