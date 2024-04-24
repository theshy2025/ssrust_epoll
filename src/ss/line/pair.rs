use crate::ss::{Line, Status, Tag};

impl Line {
    pub fn set_pair_id(&mut self,id:u64) {
        let m = format!("pair_id change from {} to {},{:?},{:?}",self.pair_id,id,self.status,self.tag);
        self.pair_id = id;
        self.log(m);
    }

    pub fn on_pair_close(&mut self,id:u64) {
        self.log(format!("your pair[{}] now close {},{:?}",id,self.pair_id,self.tag));
        self.set_pair_id(0);
        match self.tag {
            Tag::MainLand => self.set_status(Status::CoolDown),
            Tag::Hk => self.set_status(Status::Raw),
            _ => {},
        }
    }
}