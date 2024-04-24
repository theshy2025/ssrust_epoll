use std::{io::{Read, Write}, net::SocketAddrV4};

use crate::ss::{Line, Status};

impl Line {
    pub fn start_connect(&mut self) {
        let address:SocketAddrV4 = self.peer_ip.parse().unwrap();
        self.log(format!("connecting to {} [{}]",self.peer_ip,self.clock.elapsed().as_millis()));
        match self.socket.connect(&address.into()) {
            Ok(_) => todo!(),
            Err(e) => self.log(format!("{},{}",address,e)),
        }
    }

    pub fn on_connect_success(&mut self) {
        self.log(format!("on_connect_success {:?} [{}]",self.status,self.clock.elapsed().as_millis()));
        self.set_status(Status::Established);
    }

    pub fn socket_send(&mut self,buf:&[u8]) {
        let len = buf.len();
        self.log(format!("try socket_send {} bytes {:?}[{}]",len,self.status,self.clock.elapsed().as_millis()));
        match self.status {
            Status::WriteClose | Status::ReadWriteBothClose 
            | Status::DeRegister | Status::Close => {},
            
            _ => {
                match self.socket.write(buf) {
                    Ok(n) => {
                        self.log(format!("stream write {} bytes to {:?}[{}]",n,self.tag,self.clock.elapsed().as_millis()));
                        if n < buf.len() {
                            self.err(format!("[{}]some data left",self.id));
                        }
                    },
                    Err(e) => self.err(format!("[{}]stream write fail {}",self.id,e)),
                }
            }
        }
    }

    pub fn on_read_able(&mut self,buf:&mut [u8]) -> usize {
        //self.log(format!("on_read_able {:?}",self.status));
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
                self.err(format!("[{}]stream read fail {}",self.id,e));
                0
            },
        }
    }
}