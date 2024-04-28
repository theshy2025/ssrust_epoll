use std::io::Read;

use super::{network::LineNetWork, status::Status};

pub trait LineEvent: LineNetWork {

    fn on_error(&mut self) {
        let err = self.socket().take_error();
        let s = format!("on_error {:?}",err);
        self.log(s);
    }

    fn on_hang_up(&mut self) {
        self.log(format!("on_hang_up"));
        self.set_status(Status::ReadWriteBothClose);
    }

    fn on_rd_hang_up(&mut self) {
        let st = self.status();
        self.log(format!("on_rd_hang_up"));
        
        if st == Status::ReadWriteBothClose {
            return;
        }

        if st == Status::ReadClose {
            self.set_status(Status::ReadWriteBothClose);
        } else {
            self.set_status(Status::WriteClose);
        }
    }

    fn on_read_close(&mut self) {
        let st = self.status();
        self.log(format!("on_read_close"));
        if st == Status::ReadWriteBothClose {
            return;
        }

        if st == Status::WriteClose {
            self.set_status(Status::ReadWriteBothClose);
        } else {
            self.set_status(Status::ReadClose);
        }
    }

    fn on_write_able(&mut self) {
        self.set_status(Status::WriteOpen);
    }

    fn on_read_able(&mut self,buf:&mut [u8]) -> usize {
        let st = self.status();
        if st == Status::ReadClose || st == Status::ReadWriteBothClose {
            return 0;
        }

        match self.socket().read(buf) {
            Ok(n) => {
                if n > 0 {
                    self.on_network_data(&mut buf[..n])
                } else {
                    self.on_read_close();
                    0
                }
            },

            Err(e) => {
                self.log(format!("[{}]stream read fail {}",self.id(),e));
                0
            },
        }
    }

    fn turn_dead(&mut self) {
        self.set_status(Status::Dead);
    }
}

