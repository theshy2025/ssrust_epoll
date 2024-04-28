use nix::sys::epoll::EpollFlags;

use crate::{config::{BUFF_SIZE, GATE_ID}, line::mainland::LineMainLand, log::Log};

use super::Gate;

impl Gate {
    pub fn epoll_in(&mut self,id:u64) {
        match id {
            GATE_ID => self.accept_john(),
            other => self.on_read_able_event(other),
        }
    }

    pub fn on_write_able_event(&mut self,id:u64) {
        let line = self.lines.get_mut(&id).unwrap();
        line.on_write_able();
    }

    pub fn on_read_able_event(&mut self,id:u64) {
        let line = self.lines.get_mut(&id).unwrap();
        let mut buf = [0;BUFF_SIZE];
        let n = line.on_read_able(&mut buf);
        if n > 0 {
            let pid = line.pair_id();
            if pid > 0 {
                let line = self.lines.get_mut(&pid).unwrap();
                line.socket_send(&buf[..n]);
                match line.as_any_mut().downcast_mut::<LineMainLand>() {
                    Some(mainland) => mainland.send_done(n),
                    None => {},
                }
            }
        }
    }

    pub fn on_rd_hang_up_event(&mut self,id:u64) {
        let line = self.lines.get_mut(&id).unwrap();
        line.on_rd_hang_up();
    }

    pub fn epoll_err(&mut self,id:u64) {
        match id {
            GATE_ID => self.log(format!("gate error")),
            other => {
                let line = self.lines.get_mut(&other).unwrap();
                line.on_error();
            }
        }
    }

    pub fn on_hang_up_event(&mut self,id:u64) {
        let line = self.lines.get_mut(&id).unwrap();
        line.on_hang_up();
    }

    pub fn on_other_event(&mut self,id:u64,flags:EpollFlags) {
        self.log(format!("[{}]on_event {:?}",id,flags));
    }
}


/*
        match line.status {
            
            _ => {
                
                let pid = line.pair_id;
                //self.log(format!("line[{}][{}]garbage {} bytes",id,pid,n));
                
                   
                        
                    } else {
                        log::im(format!("[{}][error]can not find pair to do tcp send",id));
                    }
                }
            }
        }*/