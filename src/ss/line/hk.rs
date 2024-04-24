use crate::{config::TCP_LIFE_TIME, log, ss::{reverse, Line, Status}};

impl Line {
    pub fn on_data_from_hk(&mut self,buf:&[u8]) -> usize {
        self.log(format!("on_data_from_hk {} bytes {:?}",buf.len(),self.status));

        if buf.len() > u8::BITS as usize {
            if self.pair_id > 0 {
                return buf.len()
            } else {
                self.err(format!("data no where to go"));
                return 0;
            }
        }

        self.on_recv_server_heart_beat(buf);
        
        0
    }

    pub fn send_heart_beat(&mut self) {
        if self.pair_id > 0 {
            return;
        }

        if self.is_raw() {
            return;
        }
        
        if log::now() - self.last_send_heart_beat < TCP_LIFE_TIME/10 {
            return;
        }

        self.last_send_heart_beat = log::now();

        let mut buf = self.id.to_be_bytes();
        reverse(&mut buf);
        self.socket_send(&buf);

    }

    fn on_recv_server_heart_beat(&mut self,buf:&[u8]) -> usize {
        match buf.try_into() {
            Ok(arr) => {
                let id = u64::from_be_bytes(arr);
                self.log(format!("server id {}",id));
                self.set_status(Status::EncryptDone);
                self.last_recv_heart_beat = log::now();
                self.log(format!("on_recv_server_heart_beat {} {}",self.last_recv_heart_beat,self.clock.elapsed().as_secs()));
            },
            Err(e) => self.err(format!("{}",e)),
        };
        0
    }

    
}
