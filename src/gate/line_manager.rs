use std::os::fd::AsFd;

use crate::{global::frame, line::{tcp2vps::LineTcp2Vps, mainland::LineMainLand,status::Status}, log::{self, Log}};

use super::Gate;

impl Gate {

    pub fn clear_dead_line(&mut self) {
        let mut dead:Vec<u64> = Vec::new();
        let mut hk = 0;
        let mut not_establish = 0;
        let mut working = 0;
        let mut ready = 0;
        let mut timeout = 0;
        
        for (_,line) in self.lines.iter_mut() {
            let id = line.id();
            if line.status() == Status::Dead {
                dead.push(id);
            } else {
                match line.as_any().downcast_ref::<LineTcp2Vps>() {
                    Some(h) => {
                        hk = hk + 1;

                        if h.is_ready() {
                            ready = ready + 1;
                        } else {
                            if h.pair_id > 0 {
                                working = working + 1;
                            } else if h.tcp_timeout() {
                                timeout = timeout + 1;
                            } else if  line.status() != Status::Establish {
                                not_establish =  not_establish + 1;
                            }
                        }
                    },
                    None => {},
                }
            }
        }

        if frame() > 100 && hk > 1 && ready < 20 {
            self.log(format!("dead:{},hk:{},not_establish:{},timeout:{},working:{},ready:{}",dead.len(),hk,not_establish,timeout,working,ready));
            //panic!()
        }
        

        for id in dead {
            self.lines.remove(&id);
        }
        
    }


    pub fn decouple(&mut self) {
        let mut close:Vec<u64> = Vec::new();
        
        for (_,line) in self.lines.iter_mut() {
            if line.status() == Status::Close {
                let pid = line.pair_id();
                if pid > 0 {
                    close.push(pid);
                    line.turn_dead();
                }
            }
        }

        for pid in close.iter() {
            let line = self.lines.get_mut(pid).unwrap();
            line.on_pair_close();
        }
    }

    pub fn deregister(&mut self) {
        let mut hang_up:Vec<u64> = Vec::new();
        let mut change:Vec<u64> = Vec::new();
        let mut mainland:Vec<u64> = Vec::new();
        
        for (_,line) in self.lines.iter_mut() {
            let id = line.id();
            let pid = line.pair_id();
            let st = line.status();
            if st == Status::ReadWriteBothClose {
                hang_up.push(id);
                line.set_status(Status::Close);
            } else if st == Status::WriteOpen {
                hang_up.push(id);
                change.push(id);
                line.set_status(Status::Register);
                
                if pid > 0 {
                    mainland.push(pid);
                }
            }
        }

        for id in hang_up.iter() {
            let line = self.lines.get(id).unwrap();
            self.remove_fd( line.socket().as_fd() );
        }

        for id in change.iter() {
            let line = self.lines.get(id).unwrap();
            self.register_read_event(line.socket().as_fd(), line.id());
        }

        for id in mainland.iter() {
            let line = self.lines.get_mut(id).unwrap();
            match line.as_any_mut().downcast_mut::<LineMainLand>() {
                Some(m) => m.world_connect_success(),
                None => {},
            }
        }

    }


    pub fn gather_client_hello(&mut self) {
        let mut data:Vec<(u64,Vec<u8>)> = Vec::new();

        for (_,line) in self.lines.iter_mut() {
            match line.as_any_mut().downcast_mut::<LineMainLand>() {
                Some(mainland) => {
                    match mainland.move_out_client_hello_data() {
                        Some(buf) => {
                            let node = (line.pair_id(),buf);
                            data.push(node);
                        },
                        
                        None => {},
                    }
                },

                None => {},
            }
        }

        for (id,buf) in data.iter() {
            match self.lines.get_mut(id) {
                Some(line) => {
                    line.socket_send(buf);
                },
                None => {
                    log::err(format!("error can not find world line {}",id))
                },
            }
        }
    }

    
}

impl Gate {
    pub fn tcp_keep_alive(&mut self) {
        for (_,line) in self.lines.iter_mut() {
            match line.as_any_mut().downcast_mut::<LineTcp2Vps>() {
                Some(hk) => hk.send_heart_beat(),
                None => {},
            }
        }
    }
}

