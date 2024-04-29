use socket2::Socket;

use crate::{config, log::Log};

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
        let num = config::chick_init_num();
        if num > 0 {
            self.find_chick_for_pc(socket);
        } else {
            self.new_mainland_line(socket);
        }
    }
}