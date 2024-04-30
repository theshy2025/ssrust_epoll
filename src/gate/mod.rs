use std::{collections::HashMap, net::SocketAddrV4, os::fd::AsFd};

use nix::sys::epoll::{Epoll, EpollCreateFlags};
use socket2::{Domain, Socket, Type};

use crate::{config::{self,TCP_GATE_ID, UDP_GATE_ID}, global, line::LineTrait, log::{buf_writer::LogBufWriter, Log}};

mod epoll;
mod event;
mod dns;
mod pc;
mod line_creater;
mod line_manager;
mod john;

pub struct Gate {
    tcp_socket:Socket,
    udp_socket:Socket,
    epoll:Epoll,
    lines:HashMap<u64,Box<dyn LineTrait>>,
    buf_writer:LogBufWriter,
}

impl Gate {
    pub fn new() -> Gate {
        let tcp_socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
        let udp_socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
        let epoll = Epoll::new(EpollCreateFlags::empty()).unwrap();
        let device = config::device();
        let path = format!("{}/{}.log",device,module_path!().split("::").last().unwrap());
        let buf_writer = LogBufWriter::new(path).unwrap();

        Gate{ tcp_socket , udp_socket , epoll , lines:HashMap::new(), buf_writer }
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
    fn init_tcp_socket(&mut self) {
        let address: SocketAddrV4 = format!("0.0.0.0:{}",config::tcp_port()).parse().unwrap();
        self.tcp_socket.set_nonblocking(true).unwrap();
        self.tcp_socket.bind(&address.into()).unwrap();
        self.tcp_socket.listen(128).unwrap();
        self.register_read_event(self.tcp_socket.as_fd(), TCP_GATE_ID);
        self.log(format!("tcp socket listening on {:?}",address));
    }

    fn init_udp_socket(&mut self) {
        let address: SocketAddrV4 = format!("0.0.0.0:{}",config::udp_port()).parse().unwrap();
        self.udp_socket.bind(&address.into()).unwrap();
        let addr = self.udp_socket.local_addr().unwrap();
        let addr = addr.as_socket_ipv4().unwrap();
        self.register_read_event(self.udp_socket.as_fd(), UDP_GATE_ID);
        self.log(format!("udp socket bind to {:?}",addr));
        let port = config::vps_udp_port();
        if port > 0 {
            let address: SocketAddrV4 = format!("{}:{}",config::vps_ip(),port).parse().unwrap();
            self.udp_socket.connect(&address.into()).unwrap();
            let addr = self.udp_socket.peer_addr().unwrap();
            let addr = addr.as_socket_ipv4().unwrap();
            self.log(format!("udp socket connect to {:?}",addr));
            //self.udp_socket.send(&[1,2,5]).unwrap();
        }
        
    }


    fn init(&mut self) {
        self.init_tcp_socket();
        self.init_udp_socket();
        

        let n = config::chick_init_num();
        if n > 0 {
            self.create_hk_chicks(n);
        } else {
            self.activate_dns_manager();
        }
    }
}

impl Log for Gate {
    fn logger(&mut self) -> &mut LogBufWriter {
       &mut self.buf_writer
    }
}