use std::time::Instant;

use crate::{config::{ATYP_HOST_NAME, ATYP_INDEX}, log, ss::{reverse, u8r, Line, Status}};

impl Line {
    
    pub fn on_data_from_mainland(&mut self,buf:&mut [u8]) -> usize {
        self.log(format!("on_data_from_mainland {} bytes {:?} [{}]",buf.len(),self.status,self.clock.elapsed().as_millis()));
        let id = self.try_decode_heart_beat(buf);
        if id > 0 {
            self.recv_client_heart_beat(id);
            return 0;
        }

        match self.status {
            Status::Raw | Status::Established => self.recv_sni_msg(buf),
            Status::WaitingDnsResult | Status::DnsQuerySuccess => self.save_client_hello_data(buf),
            Status::WorldConnectSuccess => self.client_hello(buf),
            
            Status::EncryptDone => {
                if self.pair_id > 0 {
                    buf.len()
                } else {
                    self.err(format!("data had no where to go"));
                    0
                }
            },
            
            Status::CoolDown => {
                self.err(format!("data had no where to go"));
                0
            },

            _ => todo!()
        }
    }

    pub fn waiting_for_dns_result(&mut self) {
        self.set_status(Status::WaitingDnsResult);
    }

    pub fn dns_query_success(&mut self,world_id:u64) {
        let len = self.client_hello_data.len();
        self.log(format!("dns_query_success {} client_hello len {} [{}]",world_id,len,self.clock.elapsed().as_millis()));
        self.set_pair_id(world_id);
        self.set_status(Status::DnsQuerySuccess);
    }

    pub fn dns_query_fail(&mut self) {
        self.log(format!("dns_query_fail {:?}",self.peer_ip));
    }

    pub fn world_connect_success(&mut self) {
        let len = self.client_hello_data.len();
        self.log(format!("world_connect_success client_hello len {} [{}]",len,self.clock.elapsed().as_millis()));
        self.set_status(Status::WorldConnectSuccess);
    }

    pub fn move_out_client_hello_data(&mut self) -> Vec<u8> {
        self.log(format!("move_out_client_hello_data {}",self.client_hello_data.len()));
        let data = self.client_hello_data.clone();
        self.client_hello_data.clear();
        self.set_status(Status::EncryptDone);
        data
    }
    
}

impl Line {
    fn recv_client_heart_beat(&mut self,id:u64) {
        self.log(format!("client id {}",id));
        
        if self.pair_id > 0 {
            return;
        }

        match self.status {
            Status::Raw => {
                let buf = self.id.to_be_bytes();
                self.socket_send(&buf);
            },
            _ => {},
        }
    }

    fn decode_heart_beat(&self,buf:&[u8]) -> u64 {
        match buf.try_into() {
            Ok(arr) => {
                let id = u64::from_be_bytes(arr);
                id
            },
            Err(_) => 0,
        }
    }

    fn try_decode_heart_beat(&self,buf:&mut [u8]) -> u64 {
        if buf.len() != u8::BITS as usize {
            return 0;
        }
        reverse(buf);
        self.decode_heart_beat(buf)
    }
    

    fn recv_sni_msg(&mut self,buf:&mut [u8]) -> usize {
        self.clock = Instant::now();
        self.decode_host_name(buf);
        self.set_status(Status::FirstDone);
        let m = format!("[{}]{:?}:{}",self.id,self.peer_ip,self.peer_port);
        self.log(m.clone());
        log::im(m);
        0
    }

    fn decode_host_name(&mut self,buf:&mut [u8]) {
        reverse(buf);
        let atyp = buf[ATYP_INDEX];
        match atyp {
            ATYP_HOST_NAME => {
                let len = u8r(buf[ATYP_INDEX+1]) as usize;
                match String::from_utf8((&buf[ATYP_INDEX+2..ATYP_INDEX+2+len]).to_vec()) {
                    Ok(ret) => self.peer_ip = ret,
                    Err(e) => log::im(format!("[{}]{:?}",e,self.id)),
                }
        
                self.peer_port = u16::from_be_bytes([buf[ATYP_INDEX+len+2],buf[ATYP_INDEX+len+3]]);
            },

            other => {
                let m = format!("[{}]fail decode host name {},{:?}",self.id,other,buf);
                log::im(m.clone());
                self.log(m);
            },
        }
    }

    fn client_hello(&mut self,buf:&mut [u8]) -> usize {
        self.log(format!("client_hello {} bytes",buf.len()));
        self.set_status(Status::EncryptDone);
        reverse(buf);
        buf.len()
    }

    fn save_client_hello_data(&mut self,buf:&mut [u8]) -> usize {
        self.log(format!("save_client_hello_data before. {} bytes",self.client_hello_data.len()));
        for v in buf.iter() {
            self.client_hello_data.push(u8r(*v))
        }
        self.log(format!("save_client_hello_data after. {} bytes",self.client_hello_data.len()));
        0
    }

    

}
