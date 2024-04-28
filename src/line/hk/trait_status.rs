use crate::{line::status::{LineStatus, Status}, log::Log};

use super::LineHk;

impl LineStatus for LineHk {
    fn status(&self) -> Status {
        self.basic.status
    }

    fn set_status(&mut self,new:Status) {
        let old = self.basic.status;
        self.basic.status = new;
        self.log(format!("status {:?} to {:?}",old,new));
    }
}