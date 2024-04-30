

use std::mem::MaybeUninit;

use socket2::Socket;

use crate::{config, log::Log};

use super::Gate;



impl Gate {
    pub fn on_udp_packet(&mut self) {
        let mut buf: [MaybeUninit<u8>; 1024] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        let (n,addr) = self.udp_socket.recv_from(&mut buf).unwrap();
        self.log(format!("on_udp_packet {} bytes from {:?}",n,addr.as_socket_ipv4().unwrap()));

        self.udp_socket.send_to(&[5,2,1], &addr).unwrap();
    }

    pub fn accept_john(&mut self) {
        match self.tcp_socket.accept() {
            Ok((socket,_)) => self.on_john_connect(socket),
            Err(e) => {
                self.log(format!("{}",e));
            },
        }
    }
    
    fn on_john_connect(&mut self,socket:Socket) {
        let num = config::tcp_2_vps_line_init_num();
        if num > 0 {
            self.find_chick_for_pc(socket);
        } else {
            self.new_mainland_line(socket);
        }
    }
}