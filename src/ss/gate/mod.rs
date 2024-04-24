use std::{collections::HashMap, net::SocketAddrV4, os::fd::AsFd};

use nix::sys::epoll::*;
use socket2::{Domain, Socket, Type};

use crate::{config::{GATE, TCP_LIFE_TIME}, default_config::{CHICK_INIT_NUM, GATE_PORT}, log::{self, Log}};

use super::{Gate, Tag};

mod epoll;
mod dns;
mod pc;
mod line;

impl Gate {
    pub fn new() -> Gate {
        let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
        let epoll: Epoll = Epoll::new(EpollCreateFlags::empty()).unwrap();
        let logger = Log::create("gate");

        Gate{ socket , epoll , next_id:1 , lines:HashMap::new(), logger }
    }
}

impl Gate {
    pub fn start(&mut self) {
        self.init();
        loop {
            log::next_frame();
            self.network_check();
            self.poll();
            self.check_dns_result();
            self.gather_dns_query();
            self.clear_dead_line();
            self.decouple();
            self.mark_close();
            self.deregister();
            self.gather_client_hello();
        }
    }
}

impl Gate {
    fn init(&mut self) {
        let address: SocketAddrV4 = format!("0.0.0.0:{}",GATE_PORT).parse().unwrap();
        self.socket.bind(&address.into()).unwrap();
        self.socket.listen(128).unwrap();
        self.log(format!("listening on {:?}",address));

        self.register_read_event(self.socket.as_fd(), GATE);

        if CHICK_INIT_NUM > 0 {
            self.create_hk_chicks(CHICK_INIT_NUM);
        } else {
            self.activate_dns_manager();
        }
    }

    fn poll(&mut self) {
        //self.log(format!("blocking polling..."));
        let raw = EpollEvent::empty();
        let mut events = [raw;2];
        let mil = TCP_LIFE_TIME as u16 * 700;
        let timeout = EpollTimeout::from(mil);
        self.epoll.wait(&mut events, timeout).unwrap();
        for v in events {
            let id = v.data();
            for flag in v.events() {
                match flag {
                    EpollFlags::EPOLLIN => self.epoll_in(id),
                    EpollFlags::EPOLLOUT => self.on_write_able_event(id),
                    EpollFlags::EPOLLRDHUP => self.on_rd_hang_up_event(id),
                    EpollFlags::EPOLLERR => self.epoll_err(id),
                    EpollFlags::EPOLLHUP => self.on_hang_up_event(id),
                    other => self.on_event(id,other),
                }
            }
        }
    }

    fn epoll_in(&mut self,id:u64) {
        match id {
            GATE => self.accept_john(),
            other => self.on_read_able_event(other),
        }
    }

    fn epoll_err(&mut self,id:u64) {
        match id {
            GATE => log::im(format!("gate error")),
            other => self.on_error_event(other),
        }
    }

    fn accept_john(&mut self) {
        match self.socket.accept() {
            Ok((socket,_)) => self.on_john_connect(socket),
            Err(e) => println!("{}",e),
        }
    }

    fn on_john_connect(&mut self,socket:Socket) {
        if CHICK_INIT_NUM > 0 {
            self.find_chick_for_john(socket);
        } else {
            let id = self.next_id();
            self.new_line(id, 0, Tag::MainLand, socket);
        }
    }


    fn next_id(&mut self) -> u64 {
        self.next_id = self.next_id + 1;
        self.next_id
    }

    fn log(&mut self,s:String) {
        self.logger.add(s);
        self.logger.flush();
    }

    fn err(&mut self,s:String) {
        log::im(s.clone());
        self.log(s);
    }
}