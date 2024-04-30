use crate::{line::status::{LineStatus, Status}, log::Log};

use super::LineTcp2Vps;

impl LineStatus for LineTcp2Vps {
    fn status(&self) -> Status {
        self.basic.status
    }

    fn set_status(&mut self,new:Status) {
        let old = self.basic.status;
        self.basic.status = new;
        self.log(format!("status {:?} to {:?}",old,new));
    }
}