use std::os::fd::AsFd;

use nix::sys::epoll::EpollFlags;
use socket2::Socket;

use crate::{config::BUFF_SIZE, log, ss::{Gate, Line, Status, Tag}};

impl Gate {
    pub fn mark_close(&mut self) {
        for (_,line) in self.lines.iter_mut() {
            if line.is_deregister() {
                line.set_status(Status::Close);
            }
        }
    }

    pub fn decouple(&mut self) {
        let mut close:Vec<(u64,u64)> = Vec::new();
        
        for (_,line) in self.lines.iter_mut() {
            if line.is_close() {
                let pid = line.pair_id;
                if pid > 0 {
                    close.push((line.id,pid));
                    line.turn_dead();
                }
            }
        }

        for (id,pid) in close {
            match self.lines.get_mut(&pid) {
                Some(line) => line.on_pair_close(id),
                None => log::im(format!("fail to fetch close line pair {}-{}",id,pid)),
            }
        }
    }

    pub fn gather_client_hello(&mut self) {
        let mut data:Vec<(u64,Vec<u8>)> = Vec::new();

        for (_,line) in self.lines.iter_mut() {
            match line.tag {
                Tag::MainLand => {
                    match line.status {
                        Status::WorldConnectSuccess => {
                            let len = line.client_hello_data.len();
                            if len > 0 {
                                let garbage = line.move_out_client_hello_data();
                                let node = (line.pair_id,garbage);
                                data.push(node);
                            }
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        }

        for (id,buf) in data.iter() {
            match self.lines.get_mut(id) {
                Some(line) => {
                    line.socket_send(buf);
                },
                None => {
                    log::im(format!("error can not find world line {}",id))
                },
            }
        }
    }

    pub fn deregister(&mut self) {
        let mut hang_up:Vec<u64> = Vec::new();
        let mut world:Vec<u64> = Vec::new();
        let mut mainland:Vec<u64> = Vec::new();
        
        for (_,line) in self.lines.iter_mut() {
            if line.is_read_write_both_close() {
                hang_up.push(line.id);
                line.set_status(Status::DeRegister);
            } else {
                match line.tag {
                    Tag::World => {
                        if line.is_established() {
                            hang_up.push(line.id);
                            world.push(line.id);
                            mainland.push(line.pair_id);
                            line.set_status(Status::FirstPackDone);
                        }
                    },
                    _ => {},
                }
            }
        }



        for id in hang_up.iter() {
            let line = self.lines.get(id).unwrap();
            self.remove_fd( line.socket.as_fd() );
        }

        for id in world.iter() {
            let line = self.lines.get(id).unwrap();
            self.register_read_event(line.socket.as_fd(), line.id);
            
        }

        for id in mainland.iter() {
            let mainland = self.lines.get_mut(id).unwrap();
            mainland.world_connect_success();
        }

    }
    
    pub fn clear_dead_line(&mut self) {
        let mut dead:Vec<u64> = Vec::new();
        let mut hk = 0;
        let mut working = 0;
        let mut ready = 0;

        
        for (_,line) in self.lines.iter_mut() {
            if line.is_dead() {
                dead.push(line.id);
            }

            if line.is_hk_chick() {
                hk = hk + 1;
                
                if line.is_working() {
                    working = working + 1;
                }

                if line.is_ready() {
                    ready = ready + 1;
                }
                
            }
        }

        self.log(format!("dead:{},hk:{},working:{},ready:{}",dead.len(),hk,working,ready));

        for id in dead {
            self.lines.remove(&id);
        }
        
    }

    pub fn network_check(&mut self) {
        for (_,line) in self.lines.iter_mut() {
            match line.tag {
                Tag::Hk => line.send_heart_beat(),
                _ => {},
            }
        }
    }

    pub fn on_event(&mut self,id:u64,flag:EpollFlags) {
        let m = format!("line[{}]on_event {:?}",id,flag);
        log::im(m);
    }

    pub fn on_hang_up_event(&mut self,id:u64) {
        match self.lines.get_mut(&id) {
            Some(line) => {
                match line.status {
                    Status::Close | Status::ReadWriteBothClose => {},
                    _ => line.on_hang_up(),
                }
            },
            None => log::im(format!("on_rd_hang_up_event Unexpected {}",id)),
        }
    }

    pub fn on_rd_hang_up_event(&mut self,id:u64) {
        match self.lines.get_mut(&id) {
            Some(line) => {
                match line.status {
                    Status::WriteClose | Status::ReadWriteBothClose  => {},
                    _ => line.on_rd_hang_up(),
                }
            },
            None => log::im(format!("on_rd_hang_up_event Unexpected {}",id)),
        }
    }

    pub fn on_write_able_event(&mut self,id:u64) {
        let line = self.lines.get_mut(&id).unwrap();
        match line.tag {
            Tag::World => {
                line.on_connect_success();
            },
            _ => log::im(format!("[{}]on_write_able_event Unexpected",id)),
        }
    }

    pub fn on_error_event(&mut self,id:u64) {
        let line = self.lines.get_mut(&id).unwrap();
        line.on_error();
    }

    pub fn on_read_able_event(&mut self,id:u64) {
        let line = self.lines.get_mut(&id).unwrap();
        match line.status {
            Status::ReadClose | Status::ReadWriteBothClose => {},
            _ => {
                let mut buf = [0;BUFF_SIZE];
                let n = line.on_read_able(&mut buf);
                let pid = line.pair_id;
                self.log(format!("line[{}][{}]garbage {} bytes",id,pid,n));
                if n > 0 {
                    if pid > 0 {
                        let line = self.lines.get_mut(&pid).unwrap();
                        line.socket_send(&buf[..n]);
                    } else {
                        log::im(format!("[{}][error]can not find pair to do tcp send",id));
                    }
                }
            }
        }
    }

    pub fn new_line(&mut self,id:u64,pair_id:u64,tag:Tag,socket:Socket) {
        match tag {
            Tag::World => self.register_write_event(socket.as_fd(), id),
            _ => self.register_read_event(socket.as_fd(), id),
        }
        let line = Line::new(id,pair_id,tag,socket);
        self.lines.insert(id, line);
    }
    
}