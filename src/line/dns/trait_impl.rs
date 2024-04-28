use std::any::Any;

use socket2::Socket;

use crate::{config::DNS_ID, line::{event::LineEvent, network::LineNetWork, pair::LinePair, status::LineStatus, LineTrait}, log::{buf_writer::LogBufWriter, log_dir::LogDir, Log}};

use super::LineDns;




impl Log for LineDns {
    fn id(&self) -> u64 {
        DNS_ID
    }

    fn logger(&mut self) -> &mut LogBufWriter {
        &mut self.basic.buf_writer
    }
    
}

impl LineStatus for LineDns {
    
}

impl LinePair for LineDns {
    
}

impl LineEvent for LineDns {
    
}

impl LineNetWork for LineDns {
    fn socket(&self) -> &Socket {
        &self.basic.socket
    }

    fn on_network_data(&mut self,buf:&mut [u8]) -> usize {
        self.log(format!("on_data_from_dns_server {} bytes",buf.len()));
        let ret = self.decode(buf);
        self.log(format!("decode result {:?}",ret));
        self.query_result.push(ret);
        0
    }
    
}

impl LineTrait for LineDns {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl LogDir for LineDns {
    
}