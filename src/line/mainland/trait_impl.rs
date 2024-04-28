use crate::{line::{event::LineEvent, pair::LinePair, status::{LineStatus, Status}, LineTrait}, log::{log_dir::LogDir, Log}};

use super::LineMainLand;



impl Log for LineMainLand {
    fn id(&self) -> u64 {
        self.basic.id
    }

    fn logger(&mut self) -> &mut crate::log::buf_writer::LogBufWriter {
        &mut self.basic.buf_writer
    }

    fn log(&mut self,s:String) {
        let s = format!("[{}][{:?}]{}",self.pair_id(),self.status(),s);
        self.logger().add(s);
        self.logger().flush();
    }
}

impl LineStatus for LineMainLand {
    fn status(&self) -> Status {
        self.basic.status
    }

    fn set_status(&mut self,new:Status) {
        let old = self.basic.status;
        self.basic.status = new;
        self.log(format!("status {:?} to {:?}",old,new));
    }
}

impl LinePair for LineMainLand {
    fn pair_id(&self) -> u64 {
        self.pair_id
    }
    
    fn set_pair_id(&mut self,id:u64) {
        self.pair_id = id;
    }

    fn on_pair_close(&mut self) {
        self.log(format!("on_pair_close"));
        self.set_pair_id(0);
        self.set_status(Status::CoolDown);
    }
}

impl LineEvent for LineMainLand {
    
    
}

impl LineTrait for LineMainLand {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl LogDir for LineMainLand {
    
}