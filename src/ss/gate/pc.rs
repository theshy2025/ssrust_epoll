use socket2::{Domain, Socket, Type};

use crate::{default_config::SERVER_IP, log, ss::{Gate, Tag}};

impl Gate {
    pub fn create_hk_chicks(&mut self,n:u8) {
        log::create_dir(Tag::Pc);
        log::create_dir(Tag::Hk);
        for _ in 0..n {
            self.crate_one_chick();    
        }
    }

    fn crate_one_chick(&mut self) {
        let id = self.next_id();
        let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
        socket.set_nonblocking(true).unwrap();
        self.new_line(id, 0, Tag::Hk, socket);
        let line = self.lines.get_mut(&id).unwrap();
        line.set_peer_ip(SERVER_IP.to_string());
        line.start_connect();
        
    }
    /*
    let address:SocketAddrV4 = SERVER_IP.parse().unwrap();
        match socket.connect(&address.into()) {
            Ok(_) => {},
            Err(e) => self.log(format!("{},{}",address,e)),
        };
        //
        //
         */

    pub fn find_chick_for_john(&mut self,socket:Socket) {
        let chick_id = self.find_idle_hk_chick();
        if chick_id > 0 {
            let id = self.next_id();
            self.new_line(id, chick_id, Tag::Pc, socket);
            let line = self.lines.get_mut(&chick_id).unwrap();
            line.set_pair_id(id);
        } else {
            self.err(format!("no chick available"));
        }
    }

    fn find_idle_hk_chick(&self) -> u64 {
        for (_,line) in self.lines.iter() {
            if line.is_hk_chick() {
                if line.is_ready() {
                    return line.id;
                }
            }
        }
        0
    }
}