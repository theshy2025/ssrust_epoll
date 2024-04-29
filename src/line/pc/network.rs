use std::io::Write;

use socket2::Socket;

use crate::{config::{ATYP_HOST_NAME, ATYP_INDEX}, line::network::LineNetWork, log::{self, Log}};

use super::LinePc;

#[derive(Debug)]
pub enum Step {
    Raw,
    HelloDone,
    SniDone,
    ClientHelloDone,
}

impl LineNetWork for LinePc {
    fn socket(&self) -> &Socket {
        &self.basic.socket
    }

    fn on_network_data(&mut self,buf:&mut [u8]) -> usize {
        let len = buf.len();
        self.log(format!("on network data from {} {} bytes {:?}",self.peer_name(),len,self.step));
        match self.step {
            Step::Raw => self.s5_hello(),
            Step::HelloDone => self.s5_sni(buf),
            Step::SniDone => self.s5_client_hello(buf),
            Step::ClientHelloDone => buf.len(),
        }
    }
}

impl LinePc {
    fn s5_hello(&mut self) -> usize {
        self.log(format!("s5_hello"));
        self.socket().write(&[5,0]).unwrap();
        
        self.step = Step::HelloDone;

        0
    }

    fn s5_sni(&mut self,buf:&mut [u8]) -> usize {
        assert_eq!(buf[0],5);
        buf[0] = 0;
        let atyp = buf[ATYP_INDEX];
        assert_eq!(atyp,ATYP_HOST_NAME);
        let len = buf[ATYP_INDEX+1] as usize;

        let start = ATYP_INDEX+2;
        let stop = start + len;
        let vec = &buf[start..stop];
        let host = String::from_utf8(vec.to_vec());
        let port = u16::from_be_bytes([buf[stop],buf[stop+1]]);

        let s = format!("id:{} s5_sni len:{},{:?}:{}",self.id(),len,host,port);
        log::def(s.clone());
        self.log(s);

        for i in start..stop+2 {
            buf[i] = crate::global::u8r(buf[i]);
        }

        self.socket().write(&[5,0,0,1,0,0,0,0,0,0]).unwrap();

        self.step = Step::SniDone;
        
        buf.len()
    }

    fn s5_client_hello(&mut self,buf:&mut [u8]) -> usize {
        self.log(format!("s5_client_hello"));
        crate::global::reverse(buf);

        self.step = Step::ClientHelloDone;

        buf.len()
    }
}