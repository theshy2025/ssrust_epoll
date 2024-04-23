use std::net::SocketAddrV4;

use crate::ss::{Line, Status};

impl Line {
    
    pub fn on_data_from_world(&mut self,buf:&mut [u8]) -> usize {
        self.log(format!("on_data_from_world {} bytes {:?}",buf.len(),self.status));
        buf.len()
    }

    pub fn start_connect(&mut self) {
        self.log(format!("connecting to {}",self.website_host));
        let address:SocketAddrV4 = self.website_host.parse().unwrap();
        match self.socket.connect(&address.into()) {
            Ok(_) => todo!(),
            Err(e) => self.log(format!("{},{}",address,e)),
        }
    }

    pub fn on_connect_success(&mut self) {
        self.log(format!("on_connect_success {:?}",self.status));
        self.set_status(Status::Established);
    }
}