use std::any::Any;

use socket2::Socket;

use crate::{line::{event::LineEvent, network::LineNetWork, pair::LinePair, status::{LineStatus, Status}, LineTrait}, log::{buf_writer::LogBufWriter, log_dir::LogDir, Log}};

use super::LineWorld;

impl Log for LineWorld {
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

impl LineStatus for LineWorld {
    fn status(&self) -> Status {
        self.basic.status
    }

    fn set_status(&mut self,new:Status) {
        let old = self.basic.status;
        self.basic.status = new;
        self.log(format!("status {:?} to {:?}",old,new));
    }
}

impl LinePair for LineWorld {
    fn pair_id(&self) -> u64 {
        self.pair_id
    }
}

impl LineEvent for LineWorld {
    
}

impl LineNetWork for LineWorld {
    fn socket(&self) -> &Socket {
        &self.basic.socket
    }

    fn peer_ip(&self) -> String {
        self.peer_address.clone()
    }

    fn on_network_data(&mut self,buf:&mut [u8]) -> usize {
        let len = buf.len();
        self.log(format!("on network data from {} {} bytes",self.peer_name(),len));
        len
    }
}

impl LineTrait for LineWorld {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl LogDir for LineWorld {
    
}