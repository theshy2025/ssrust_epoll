use crate::{config::TCP_LIFE_TIME, log::{self, frame}, ss::{Line, Status, Tag}};

impl Line {
    pub fn set_status(&mut self,new:Status) {
        self.log(format!("change status from {:?} to {:?} ",self.status,new));
        self.status = new;
    }

    pub fn on_read_close(&mut self) {
        self.log(format!("on_read_close {:?},{:?}",self.status,self.tag));
        match self.status {
            Status::WriteClose => self.set_status(Status::ReadWriteBothClose),
            _ => self.set_status(Status::ReadClose),
        }
    }

    pub fn on_hang_up(&mut self) {
        self.log(format!("on_hang_up {:?}",self.status));
        self.set_status(Status::ReadWriteBothClose);
    }

    pub fn on_rd_hang_up(&mut self) {
        self.log(format!("on_rd_hang_up {:?}",self.status));
        match self.status {
            Status::ReadClose => self.set_status(Status::ReadWriteBothClose),
            _ => self.set_status(Status::WriteClose),
        }
    }

    pub fn on_error(&mut self) {
        let err = self.socket.take_error().unwrap();
        self.err(format!("on_error {:?} {:?}",self.status,err));
    }

    pub fn turn_dead(&mut self) {
        let kb = self.traffic/1024;
        self.err(format!("turn_dead {:?} {}kb",self.status,kb));
        self.set_pair_id(0);
        self.set_status(Status::Dead);
    }

}

impl Line {

    pub fn is_hk_chick(&self) -> bool {
        match self.tag {
            Tag::Hk => true,
            _ => false,
        }
    }

    pub fn is_raw(&self) -> bool {
        match self.status {
            Status::Raw => true,
            _ => false,
        }
    }

    pub fn is_established(&self) -> bool {
        match self.status {
            Status::Established => true,
            _ => false,
        }
    }

    pub fn is_encrypt_done(&self) -> bool {
        match self.status {
            Status::EncryptDone => true,
            _ => false,
        }
    }

    pub fn is_working(&self) -> bool {
        if self.pair_id == 0 {
            return false;
        }

        match self.status {
            Status::Established | Status::FirstDone | 
            Status::SecondDone | Status::EncryptDone => true,
            
            _ => false,
        }
    }

    pub fn tcp_active(&self) -> bool {
        let gap = log::now() - self.last_recv_heart_beat;
        if gap < TCP_LIFE_TIME {
            return true;
        }
        
        if frame() > 100 {
            //log::im(format!("[{}]{:?} {}",self.id,self.status,gap));
        }

        false
    }

    pub fn is_ready(&self) -> bool {
        if self.pair_id > 0 {
            return false;
        }
        
        if !self.tcp_active() {
            return false;
        }

        if self.is_encrypt_done() {
            return true;
        }

        false
    }

    pub fn is_read_write_both_close(&self) -> bool {
        match self.status {
            Status::ReadWriteBothClose => true,
            _ => false,
        }
    }

    pub fn is_deregister(&self) -> bool {
        match self.status {
            Status::DeRegister => true,
            _ => false,
        }
    }

    pub fn is_close(&self) -> bool {
        match self.status {
            Status::Close => true,
            _ => false,
        }
    }

    pub fn is_dead(&self) -> bool {
        match self.status {
            Status::Dead => true,
            _ => false,
        }
    }
}