use socket2::{Domain, Socket, Type};

use crate::{line::{tcp2vps::LineTcp2Vps, pc::LinePc}, log::log_dir::LogDir};

use super::Gate;

impl Gate {
    pub fn create_tcp_2_vps_lines(&mut self,n:u8) {
        LineTcp2Vps::create_dir();
        LinePc::create_dir();
        for _ in 0..n {
            self.crate_a_tcp_2_vps_line();
        }
    }

    fn crate_a_tcp_2_vps_line(&mut self) -> u64 {
        let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
        let id = self.new_tcp_2_vps_line(socket);
        let line = self.lines.get_mut(&id).unwrap();
        line.start_connect();
        id
    }

    pub fn find_chick_for_pc(&mut self,socket:Socket) {
        let chick_id = self.find_idle_hk_chick();
        if chick_id > 0 {
            let id = self.new_pc_line(chick_id,socket);
            let line = self.lines.get_mut(&chick_id).unwrap();
            line.set_pair_id(id);
        }
    }

    fn find_idle_hk_chick(&self) -> u64 {
        for (_,line) in self.lines.iter() {
            match line.as_any().downcast_ref::<LineTcp2Vps>() {
                Some(hk) => {
                    if hk.is_ready() {
                        return line.id();
                    }
                },
                None => {},
            }
        }
        0
    }
}

