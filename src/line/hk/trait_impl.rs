use std::{any::Any, time::Instant};

use socket2::Socket;

use crate::{config, line::{event::LineEvent, network::LineNetWork, pair::LinePair, status::{LineStatus, Status}, LineTrait}, log::{buf_writer::LogBufWriter, log_dir::LogDir, Log}};

use super::LineHk;

impl Log for LineHk {
    fn id(&self) -> u64 {
        self.basic.id
    }

    fn logger(&mut self) -> &mut LogBufWriter {
        &mut self.basic.buf_writer
    }
    
    fn log(&mut self,s:String) {
        let s = format!("[{}][{:?}]{}",self.pair_id(),self.status(),s);
        self.logger().add(s);
        self.logger().flush();
    }
    
}

impl LinePair for LineHk {
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



impl LineNetWork for LineHk {
    fn socket(&self) -> &Socket {
        &self.basic.socket
    }

    fn peer_ip(&self) -> String {
        config::remote_address()
    }

    fn on_network_data(&mut self,buf:&mut [u8]) -> usize {
        let len = buf.len();
        let t = self.clock.elapsed().as_millis();
        if t > 1000 {
            self.clock = Instant::now();
            self.speed = 0;
        }
        self.speed = self.speed + len;
        
        self.log(format!("on network data from {} {} bytes [{}]ms[{}]k",self.peer_name(),len,t,self.speed/1024));
        if len > u8::BITS as usize {
            if self.pair_id > 0 {
                return buf.len()
            } else {
                self.log(format!("data no where to go"));
                return 0;
            }
        }

        self.on_recv_server_heart_beat(buf);

        0
    }

}

impl LineEvent for LineHk {

    
}

impl LineTrait for LineHk {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl LogDir for LineHk {
    
}