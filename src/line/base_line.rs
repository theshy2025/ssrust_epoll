use socket2::Socket;

use crate::log::buf_writer::LogBufWriter;

use super::status::Status;

pub struct BaseLine {
    pub id:u64,
    pub status:Status,
    pub socket:Socket,
    pub buf_writer:LogBufWriter,
}

impl BaseLine {
    pub fn new(id:u64,socket:Socket,buf_writer:LogBufWriter) -> BaseLine {
        //socket.set_send_buffer_size(1024*1024*6).unwrap();
        socket.set_nonblocking(true).unwrap();
        BaseLine { id, socket, buf_writer, status: Status::Raw }
    }
}