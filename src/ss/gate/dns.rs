use std::net::SocketAddrV4;

use socket2::{Domain, Socket, Type};

use crate::{config::DNS, default_config::SERVER_IP, log, ss::{Gate, Status, Tag}};

impl Gate {
    pub fn activate_dns_manager(&mut self) {
        log::create_dir(Tag::Dns);
        log::create_dir(Tag::MainLand);
        log::create_dir(Tag::World);

        let socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
        let address:SocketAddrV4 = SERVER_IP.parse().unwrap();

        match socket.connect(&address.into()) {
            Ok(_) => {
                self.new_line(DNS, 0, Tag::Dns, socket);
            },
            Err(e) => self.log(format!("unbale connect to {},{}",address,e)),
        };
    }

    pub fn gather_dns_query(&mut self) {
        let mut names:Vec<(u64,String)> = Vec::new();

        for (_,line) in self.lines.iter_mut() {
            match line.tag {
                Tag::MainLand => {
                    match line.status {
                        Status::FirstPackDone => {
                            names.push((line.id,line.website_host.clone()));
                            line.waiting_for_dns_result();
                        },
                        _ => {},
                    }
                }
                _ => {},
            }
        }

        for (id,host) in names {
            let line = self.lines.get_mut(&DNS).unwrap();
            line.new_dns_query(id, host);
        }
    }

    pub fn check_dns_result(&mut self) {
        match self.lines.get_mut(&DNS) {
            Some(line) => {
                let mut data = line.move_out_dns_result();
                for _ in 0..data.len() {
                    let (id,ip) = data.pop().unwrap();
                    self.on_dns_result(id, ip);
                }
            },
            None => {},
        }
    }

    pub fn on_dns_result(&mut self,id:u64,ret:Option<String>) {
        let line = self.lines.get_mut(&id).unwrap();
        let host_name = line.website_host.clone();
        let port = line.website_port;
        match ret {
            Some(ip) => {
                let ip_address = format!("{}:{}",ip,port);
                let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
                socket.set_nonblocking(true).unwrap();
                let world_id: u64 = self.next_id();
                self.new_line(world_id, id, Tag::World, socket);
                let line = self.lines.get_mut(&id).unwrap();
                line.dns_query_success(world_id);
                let world_line = self.lines.get_mut(&world_id).unwrap();
                world_line.set_website_host(host_name);
                world_line.set_website_host(ip_address);
                world_line.start_connect();
            }
            None => line.dns_query_fail(),
        }
    }

}

/*

                




                match socket.connect(&address.into()) {
                    Ok(_) => todo!(),
                    Err(e) => self.log(format!("[{}]{},{}",id,address,e)),
                }




match  {
                    Ok(_) => {
                        
                        
                        
                        
                        line.connect_to_world_success(world_id);
                        let data = line.move_out_client_hello_data();
                        if data.len() > 0 {
                            let world_line = self.lines.get_mut(&world_id).unwrap();
                            world_line.socket_send(&data);
                        }
                    },
                    
                } */