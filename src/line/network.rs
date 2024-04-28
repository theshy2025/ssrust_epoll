use std::{io::Write, net::SocketAddrV4};

use socket2::Socket;

use super::{pair::LinePair, status::Status};

pub trait LineNetWork : LinePair {
    fn on_network_data(&mut self,_buf:&mut [u8]) -> usize {0}
    
    fn peer_name(&self) -> String {
        let ret:Vec<&str> = std::any::type_name::<Self>().rsplit("::").collect();
        ret[1].to_string()
    }

    fn socket(&self) -> &Socket;
    
    fn peer_ip(&self) -> String {
        String::new()
    }

    fn start_connect(&mut self) {
        let ip = self.peer_ip();
        self.log(format!("start connecting to {} ",ip));
        let address:SocketAddrV4 = ip.parse().unwrap();
        match self.socket().connect(&address.into()) {
            Ok(_) => todo!(),
            Err(e) => self.log(format!("{},{}",address,e)),
        }
    }

    fn socket_send(&mut self,buf:&[u8]) {
        let st = self.status();
        self.log(format!("try socket_send {} bytes to {}",buf.len(),self.peer_name()));
        
        if st == Status::WriteClose || st == Status::ReadWriteBothClose || 
        st == Status::DeRegister || st == Status::Close {
            return;
        }
        
        match self.socket().write(buf) {
            Ok(n) => {
                if n < buf.len() {
                    self.log(format!("some data left"));
                }
            },
            Err(e) => self.log(format!("stream write fail {}",e)),
        }
    }

}