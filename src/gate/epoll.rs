use std::os::fd::BorrowedFd;

use nix::sys::epoll::{EpollEvent,EpollFlags,EpollTimeout};

use crate::{config::TCP_LIFE_TIME, log::{self, Log}};

use super::Gate;

impl Gate {
    pub fn poll(&mut self) {
        let raw = EpollEvent::empty();
        let mut events = [raw;32];
        let mil = (TCP_LIFE_TIME*100) as u16;
        let timeout = EpollTimeout::from(mil);
        self.epoll.wait(&mut events, timeout).unwrap();
        
        let mut count = 0;
        
        for v in events {
            let id = v.data();
            if id > 0 {
                count = count + 1;
                for flags in v.events() {
                    match flags {
                        EpollFlags::EPOLLIN => self.epoll_in(id),
                        EpollFlags::EPOLLOUT => self.on_write_able_event(id),
                        EpollFlags::EPOLLRDHUP => self.on_rd_hang_up_event(id),
                        EpollFlags::EPOLLERR => self.epoll_err(id),
                        EpollFlags::EPOLLHUP => self.on_hang_up_event(id),
                        other => self.on_other_event(id,other),
                    }
                }
            }
        }

        if count > 10 {
            self.log(format!("event num {}",count));
        }
    }

    pub fn register_read_event(&self,fd:BorrowedFd<'_>,id:u64) {
        let mut flags = EpollFlags::empty();
        flags.insert(EpollFlags::EPOLLIN);
        flags.insert(EpollFlags::EPOLLRDHUP);
        self.add_fd(fd, id, flags);
    }

    pub fn register_write_event(&self,fd:BorrowedFd<'_>,id:u64) {
        let mut flags = EpollFlags::empty();
        flags.insert(EpollFlags::EPOLLOUT);
        self.add_fd(fd, id, flags);
    }

    pub fn add_fd(&self,fd:BorrowedFd<'_>,id:u64,flags:EpollFlags) {
        let event = EpollEvent::new(flags,id);
        
        match self.epoll.add(fd, event) {
            Ok(_) => {
                //let str = flags_str_name(flags);
                //log::im(format!("[{}]add_fd {:?} success {}",id,fd,str));
            },

            Err(e) => log::err(format!("[{}]add_fd {:?} fail {}",id,fd,e)),
        }
    }

    pub fn remove_fd(&self,fd:BorrowedFd<'_>) {
        match self.epoll.delete(fd) {
            Ok(_) => {
                //log::im(format!("remove_fd {:?} success",fd));
            },
            
            Err(e) => log::err(format!("remove_fd {:?} fail {}",fd,e)),
        }
    }
}