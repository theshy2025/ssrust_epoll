use std::{collections::HashMap, net::SocketAddrV4, os::fd::AsFd};

use nix::sys::epoll::{Epoll, EpollCreateFlags};
use socket2::{Domain, Socket, Type};

use crate::{config::{self,GATE_ID}, global, line::LineTrait, log::{buf_writer::LogBufWriter, Log}};

mod epoll;
mod event;
mod dns;
mod pc;
mod line_creater;
mod line_manager;
mod john;

pub struct Gate {
    socket:Socket,
    epoll:Epoll,
    lines:HashMap<u64,Box<dyn LineTrait>>,
    buf_writer:LogBufWriter,
}

impl Gate {
    pub fn new() -> Gate {
        let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
        let epoll = Epoll::new(EpollCreateFlags::empty()).unwrap();
        let device = config::device();
        let path = format!("{}/{}.log",device,module_path!().split("::").last().unwrap());
        let buf_writer = LogBufWriter::new(path).unwrap();

        Gate{ socket , epoll , lines:HashMap::new(), buf_writer }
    }
}

impl Gate {
    pub fn start(&mut self) {
        self.init();
        loop {
            global::next_frame();
            self.tcp_keep_alive();
            self.poll();
            self.clear_dead_line();
            self.check_dns_result();
            self.gather_dns_query();
            self.decouple();
            self.deregister();
            self.gather_client_hello();
        }
    }
}

impl Gate {
    fn init(&mut self) {
        let address: SocketAddrV4 = format!("0.0.0.0:{}",config::gate_port()).parse().unwrap();
        self.socket.set_nonblocking(true).unwrap();
        self.socket.bind(&address.into()).unwrap();
        self.socket.listen(128).unwrap();

        self.log(format!("listening on {:?}",address));

        self.register_read_event(self.socket.as_fd(), GATE_ID);

        let n = config::chick_init_num();
        if n > 0 {
            self.create_hk_chicks(n);
        } else {
            self.activate_dns_manager();
        }
    }
}

impl Log for Gate {
    fn id(&self) -> u64 {
        GATE_ID
    }

    fn logger(&mut self) -> &mut LogBufWriter {
       &mut self.buf_writer
    }
}