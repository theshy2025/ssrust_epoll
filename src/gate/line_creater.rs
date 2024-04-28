use std::os::fd::AsFd;

use socket2::{Domain, Socket, Type};

use crate::{config::DNS_ID, line::{dns::LineDns, hk::LineHk, mainland::LineMainLand, pc::LinePc, world::LineWorld}};

use super::Gate;

impl Gate {
    pub fn new_dns_line(&mut self,socket:Socket) {
        self.register_read_event(socket.as_fd(), DNS_ID);
        let line = LineDns::new(socket);
        self.lines.insert(DNS_ID, Box::new(line));
    }

    pub fn new_hk_line(&mut self,socket:Socket) -> u64 {
        let id = crate::global::next_id();
        self.register_write_event(socket.as_fd(), id);
        let line = LineHk::new(id,socket);
        self.lines.insert(id, Box::new(line));
        id
    }

    pub fn new_pc_line(&mut self,pair_id:u64,socket:Socket) -> u64 {
        let id: u64 = crate::global::next_id();
        self.register_read_event(socket.as_fd(), id);
        let line = LinePc::new(id,pair_id,socket);
        self.lines.insert(id, Box::new(line));
        id
    }

    pub fn new_mainland_line(&mut self,socket:Socket) -> u64 {
        let id: u64 = crate::global::next_id();
        self.register_read_event(socket.as_fd(), id);
        let mut line = LineMainLand::new(id,socket);
        line.send_heart_beat();
        self.lines.insert(id, Box::new(line));
        id
    }

    pub fn new_world_line(&mut self,pair_id:u64) -> u64 {
        let id: u64 = crate::global::next_id();
        let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
        self.register_write_event(socket.as_fd(), id);
        let line = LineWorld::new(id,pair_id,socket);
        self.lines.insert(id, Box::new(line));
        id
    }
    
    
}