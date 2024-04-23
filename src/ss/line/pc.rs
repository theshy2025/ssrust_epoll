use std::io::Write;

use crate::{config::ATYP_INDEX, log, ss::{u8r, Line, Status}};

impl Line {
    pub fn on_data_from_pc(&mut self,buf:&mut [u8]) -> usize {
        self.log(format!("on_data_from_pc {} bytes {:?}",buf.len(),self.status));
        match self.status {
            Status::Raw => self.s5_hello(),
            Status::FirstPackDone => self.s5_sni(buf),
            Status::SecondPackDone => self.s5_client_hello(buf),
            Status::EncryptDone => buf.len(),
            _ => todo!()
        }
    }
}

impl Line {
    fn s5_hello(&mut self) -> usize {
        self.log(format!("s5_hello {:?}",self.status));
        self.socket.write(&[5,0]).unwrap();
        self.set_status(Status::FirstPackDone);
        0
    }

    fn s5_sni(&mut self,buf:&mut [u8]) -> usize {
        assert_eq!(buf[0],5);
        buf[0] = 0;
        let atyp = buf[ATYP_INDEX];
        assert_eq!(atyp,3);
        let len = buf[ATYP_INDEX+1] as usize;

        let start = ATYP_INDEX+2;
        let stop = start + len;
        let vec = &buf[start..stop];
        let host = String::from_utf8(vec.to_vec());
        let port = u16::from_be_bytes([buf[stop],buf[stop+1]]);

        let m = format!("[{}]s5_sni len:{},{:?}:{} {:?}",self.id,len,host,port,self.status);
        self.log(m.clone());
        log::im(m);

        for i in start..stop+2 {
            buf[i] = u8r(buf[i]);
        }

        self.socket.write(&[5,0,0,1,0,0,0,0,0,0]).unwrap();
        self.set_status(Status::SecondPackDone);
        
        buf.len()
    }

    fn s5_client_hello(&mut self,buf:&mut [u8]) -> usize {
        for i in 0..buf.len() {
            buf[i] = u8r(buf[i]);
        }
        self.set_status(Status::EncryptDone);
        buf.len()
    }
}