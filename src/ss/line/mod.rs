use std::io::{Read, Write};

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

impl Line {
    pub fn new(id:u64,pair_id:u64,tag:Tag,socket:Socket) -> Line {
        let logger = Log::create_for_line(&tag, id);
        Line{ id , tag , socket, status: Status::Raw, 
            logger, pair_id,
            website_host: String::new(), website_port: 0,
            client_hello_data: Vec::new(),
            dns_result: Vec::new(),
            last_recv_heart_beat: 0,
            last_send_heart_beat: 0,
        }
    }
}

impl Line {
    pub fn socket_send(&mut self,buf:&[u8]) {
        match self.socket.write(buf) {
            Ok(n) => self.log(format!("stream write {} bytes {}",n,buf.len())),
            Err(e) => {
                let m = format!("[{}]stream write fail {}",self.id,e);
                self.log(m.clone());
                log::im(m);
            },
        }
    }

    pub fn on_read_able(&mut self,buf:&mut [u8]) -> usize {
        self.log(format!("on_read_able {:?}",self.status));
        match self.socket.read(buf) {
            Ok(n) => {
                if n > 0 {
                    self.on_network_data(&mut buf[..n])
                } else {
                    self.on_read_close();
                    0
                }
            },
            Err(e) => {
                let m = format!("[{}]stream read fail {}",self.id,e);
                self.log(m.clone());
                log::im(m);
                0
            },
        }
    }

    pub fn turn_dead(&mut self) {
        self.log(format!("turn_dead {:?} {}",self.status,self.pair_id));
        self.set_pair_id(0);
        self.set_status(Status::Dead);
    }

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