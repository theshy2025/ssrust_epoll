use std::os::fd::BorrowedFd;

use nix::sys::epoll::{EpollEvent, EpollFlags};

use crate::{log, ss::Gate};

impl Gate {
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
                let mut str = String::new();
                for (s,_) in flags.iter_names() {
                    str.push_str(s);
                    str.push(',');
                }
                log::im(format!("[{}]add_fd {:?} success {}",id,fd,str));
            },

            Err(e) => log::im(format!("[{}]add_fd {:?} fail {}",id,fd,e)),
        }
    }

    pub fn remove_fd(&self,fd:BorrowedFd<'_>) {
        match self.epoll.delete(fd) {
            Ok(_) => log::im(format!("remove_fd {:?} success",fd)),
            Err(e) => log::im(format!("remove_fd {:?} fail {}",fd,e)),
        }
    }
}

/*


        
        flags.insert(EpollFlags::EPOLLOUT);
        flags.insert(EpollFlags::EPOLLRDHUP);


 */