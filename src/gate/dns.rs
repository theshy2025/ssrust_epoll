use std::net::SocketAddrV4;

use socket2::{Domain, Socket, Type};

use crate::{config::DNS_ID, default_config::SERVER_IP, line::{dns::LineDns, mainland::{network::Step, LineMainLand}, world::LineWorld}, log::{log_dir::LogDir, Log}};

use super::Gate;

impl Gate {
    pub fn activate_dns_manager(&mut self) {
        LineDns::create_dir();
        LineMainLand::create_dir();
        LineWorld::create_dir();

        let socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
        let address:SocketAddrV4 = SERVER_IP.parse().unwrap();

        match socket.connect(&address.into()) {
            Ok(_) => {
                self.new_dns_line(socket);
            },
            Err(e) => self.log(format!("unbale connect to {},{}",address,e)),
        };
    }

    pub fn check_dns_result(&mut self) {
        match self.lines.get_mut(&DNS_ID) {
            Some(line) => {
                let dns = line.as_any_mut().downcast_mut::<LineDns>().unwrap();
                let mut data = dns.move_out_dns_result();
                for _ in 0..data.len() {
                    let (id,ip) = data.pop().unwrap();
                    self.on_dns_result(id, ip);
                }
            },
            None => {},
        }
    }

    fn on_dns_result(&mut self,id:u64,ret:Option<String>) {
        let line = self.lines.get_mut(&id).unwrap();
        let mainland = line.as_any_mut().downcast_mut::<LineMainLand>().unwrap();
        let host_name = mainland.peer_ip.clone();
        let port = mainland.peer_port;
        match ret {
            Some(ip) => {
                let ip_address = format!("{}:{}",ip,port);
                let world_id = self.new_world_line(id);
                
                let line = self.lines.get_mut(&id).unwrap();
                let mainland = line.as_any_mut().downcast_mut::<LineMainLand>().unwrap();
                mainland.dns_query_success(world_id);
                
                let line = self.lines.get_mut(&world_id).unwrap();
                
                let world = line.as_any_mut().downcast_mut::<LineWorld>().unwrap();
                world.set_peer_address(host_name);
                world.set_peer_address(ip_address);
                
                line.start_connect();
            }

            None => mainland.dns_query_fail(),
        }
    }

    pub fn gather_dns_query(&mut self) {
        let mut names:Vec<(u64,String)> = Vec::new();

        for (_,line) in self.lines.iter_mut() {
            match line.as_any_mut().downcast_mut::<LineMainLand>() {
                Some(mainland) => {
                    if mainland.step == Step::WaitingDnsCollect {
                        names.push((mainland.id(),mainland.peer_ip.clone()));
                        mainland.step = Step::WaitingDnsResult;
                    }
                },
                None => {},
            }
        }

        for (id,host) in names {
            let line = self.lines.get_mut(&DNS_ID).unwrap();
            let dns = line.as_any_mut().downcast_mut::<LineDns>().unwrap();
            dns.new_dns_query(id, host);
        }
    }

}

