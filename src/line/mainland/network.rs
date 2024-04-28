use socket2::Socket;

use crate::{config::{ATYP_HOST_NAME, ATYP_INDEX}, global::u8r, line::{network::LineNetWork, pair::LinePair}, log::{self, Log}};

use super::LineMainLand;

#[derive(Debug,PartialEq, Eq)]
pub enum Step {
    Raw,
    WaitingDnsCollect,
    WaitingDnsResult,
    DnsQuerySuccess,
    WorldConnectSuccess,
    ClientHelloDone,
}

impl LineNetWork for LineMainLand {
    fn socket(&self) -> &Socket {
        &self.basic.socket
    }

    fn on_network_data(&mut self,buf:&mut [u8]) -> usize {
        let len = buf.len();
        self.log(format!("on network data from {} {} bytes {:?}",self.peer_name(),len,self.step));
        
        let id = self.try_decode_heart_beat(buf);
        if id > 0 {
            self.recv_heart_beat(id);
            return 0;
        }
        match self.step {
            Step::Raw => self.recv_sni_msg(buf),
            Step::WaitingDnsCollect | Step::WaitingDnsResult | Step::DnsQuerySuccess => self.save_client_hello_data(buf),
            Step::WorldConnectSuccess => self.client_hello(buf),
            
            Step::ClientHelloDone => {
                if self.pair_id > 0 {
                    buf.len()
                } else {
                    self.log(format!("data had no where to go"));
                    0
                }
            },
            
        }
    }
}


impl LineMainLand {
    fn recv_sni_msg(&mut self,buf:&mut [u8]) -> usize {
        let buf_len = buf.len();
        let len = self.decode_host_name(buf);
        let m = format!("id:[{}] len:{},buf_len:{},{:?}:{}",self.id(),len,buf_len,self.peer_ip,self.peer_port);
        self.log(m);
        
        if buf_len > len {
            self.save_client_hello_data(&mut buf[len..]);
        }

        self.step = Step::WaitingDnsCollect;
        0
    }

    fn save_client_hello_data(&mut self,buf:&mut [u8]) -> usize {
        self.log(format!("save_client_hello_data before. {} bytes",self.client_hello_data.len()));
        for v in buf.iter() {
            self.client_hello_data.push(u8r(*v))
        }
        self.log(format!("save_client_hello_data after. {} bytes",self.client_hello_data.len()));
        0
    }


    fn client_hello(&mut self,buf:&mut [u8]) -> usize {
        self.log(format!("client_hello {} bytes",buf.len()));
        crate::global::reverse(buf);
        self.step = Step::ClientHelloDone;
        buf.len()
    }


    fn decode_host_name(&mut self,buf:&mut [u8]) -> usize {
        let buf_len = buf.len();
        let mut stop = 0;
        
        let atyp = buf[ATYP_INDEX];
        
        match atyp {
            ATYP_HOST_NAME => {
                let len = buf[ATYP_INDEX+1] as usize;
                stop = ATYP_INDEX+len+3;
                if buf_len <= stop {
                    log::err(format!("[{}]{},{},{},{:?}",self.id(),buf_len,stop,len,buf));
                }
                let mut vec:Vec<u8> = Vec::new();
                
                for i in ATYP_INDEX+2..ATYP_INDEX+2+len {
                    vec.push(u8r(buf[i]));
                }

                match String::from_utf8(vec) {
                    Ok(ret) => self.peer_ip = ret,
                    Err(e) => log::err(format!("[{}]{:?}",e,self.id())),
                }
                
                let p1 = u8r(buf[stop-1]);
                let p2 = u8r(buf[stop]);
                self.peer_port = u16::from_be_bytes([p1,p2]);

            },

            other => {
                let m = format!("fail decode host name {},{:?}",other,buf);
                self.log(m);
            },
        }

        stop+1
        
    }

}

impl LineMainLand {
    pub fn dns_query_fail(&mut self) {
        self.log(format!("dns_query_fail {:?}",self.peer_ip));
    }
    
    pub fn dns_query_success(&mut self,world_id:u64) {
        let len = self.client_hello_data.len();
        self.log(format!("dns_query_success {} client_hello len {}",world_id,len));
        self.set_pair_id(world_id);
        self.step = Step::DnsQuerySuccess;
    }

    pub fn world_connect_success(&mut self) {
        let len = self.client_hello_data.len();
        self.log(format!("world_connect_success client_hello len {}",len));
        self.step = Step::WorldConnectSuccess;
    }

    pub fn move_out_client_hello_data(&mut self) -> Option<Vec<u8>> {
        
        if self.step != Step::WorldConnectSuccess {
            return None;
        }

        let len = self.client_hello_data.len();
        
        if len > 0 {
            let data = self.client_hello_data.clone();
            self.client_hello_data.clear();
            self.log(format!("move_out_client_hello_data {}",len));
            self.step = Step::ClientHelloDone;
            Some(data)
        } else {
            None
        }
    }
}