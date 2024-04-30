use crate::{config::TCP_LIFE_TIME, global, line::{network::LineNetWork, status::{LineStatus, Status}}, log::Log};

use super::LineTcp2Vps;

impl LineTcp2Vps {
    pub fn send_heart_beat(&mut self) {
        if self.pair_id > 0 {
            return;
        }

        if global::now() - self.last_send_heart_beat < TCP_LIFE_TIME/10 {
            return;
        }

        if global::now() - self.last_recv_heart_beat < TCP_LIFE_TIME/10 {
            return;
        }

        if self.status() == Status::Establish || self.status() == Status::CoolDown {
            self.last_send_heart_beat = global::now();
            let mut buf = self.id().to_be_bytes();
            crate::global::reverse(&mut buf);
            self.socket_send(&buf);
        }
    }

    pub fn on_recv_server_heart_beat(&mut self,buf:&[u8]) -> usize {
        match buf.try_into() {
            Ok(arr) => {
                let id = u64::from_be_bytes(arr);
                self.log(format!("server id {}",id));
                self.set_status(Status::Establish);
                self.last_recv_heart_beat = global::now();
            },
            Err(e) => self.log(format!("{}",e)),
        };
        0
    }
}