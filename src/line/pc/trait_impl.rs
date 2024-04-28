use std::any::Any;

use crate::{line::{event::LineEvent, pair::LinePair, status::{LineStatus, Status}, LineTrait}, log::{buf_writer::LogBufWriter, log_dir::LogDir, Log}};

use super::LinePc;

impl Log for LinePc {
    fn logger(&mut self) -> &mut LogBufWriter {
        &mut self.basic.buf_writer
    }

    fn id(&self) -> u64 {
        self.basic.id
    }

    fn log(&mut self,s:String) {
        let s = format!("[{}][{:?}]{}",self.pair_id,self.status(),s);
        self.logger().add(s);
        self.logger().flush();
    }
}

impl LineStatus for LinePc {
    fn status(&self) -> Status {
        self.basic.status
    }

    fn set_status(&mut self,new:Status) {
        let old = self.basic.status;
        self.basic.status = new;
        self.log(format!("status {:?} to {:?}",old,new));
    }
}

impl LinePair for LinePc {
    fn pair_id(&self) -> u64 {
        self.pair_id
    }
    
    fn set_pair_id(&mut self,new:u64) {
        self.pair_id = new;
    }
}

impl LineEvent for LinePc {
    
}

impl LineTrait for LinePc {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl LogDir for LinePc {
    
}