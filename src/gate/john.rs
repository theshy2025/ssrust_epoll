use socket2::Socket;

use crate::{default_config::CHICK_INIT_NUM, log::Log};

use super::Gate;

impl Gate {
    pub fn accept_john(&mut self) {
        match self.socket.accept() {
            Ok((socket,_)) => self.on_john_connect(socket),
            Err(e) => {
                self.log(format!("{}",e));
            },
        }
    }
    
    fn on_john_connect(&mut self,socket:Socket) {
        if CHICK_INIT_NUM > 0 {
            self.find_chick_for_pc(socket);
        } else {
            self.new_mainland_line(socket);
        }
    }
}