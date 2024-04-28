use crate::{line::{network::LineNetWork, status::{LineStatus, Status}}, log::Log};

use super::{network::Step, LineMainLand};

impl LineMainLand {
    pub fn try_decode_heart_beat(&self,buf:&mut [u8]) -> u64 {
        if buf.len() != u8::BITS as usize {
            return 0;
        }
        crate::global::reverse(buf);
        self.decode_heart_beat(buf)
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

    
    
    pub fn recv_heart_beat(&mut self,id:u64) {
        if self.pair_id > 0 {
            return;
        }

        self.log(format!("client id {}",id));

        match self.status() {
            Status::Raw => {
                self.send_heart_beat();
                self.set_status(Status::Establish);
                self.step = Step::Raw;
            }

            Status::Establish => self.send_heart_beat(),
            
            Status::CoolDown => self.set_status(Status::Raw),
            
            _ => {},
        }
    }


    pub fn send_heart_beat(&mut self) {
        let buf = self.id().to_be_bytes();
        self.socket_send(&buf);
    }

}