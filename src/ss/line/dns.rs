use simple_dns::*;
use crate::ss::Line;

impl Line {
    pub fn on_data_from_dns_server(&mut self,buf:&mut [u8]) -> usize {
        self.log(format!("on_data_from_dns_server {} bytes {:?}",buf.len(),self.status));
        let ret =self.decode(buf);
        self.log(format!("decode result {:?}",ret));
        self.dns_result.push(ret);
        0
    }

    pub fn new_dns_query(&mut self,id:u64,host:String) {
        self.log(format!("new_dns_query {} {:?}",id,host));
        let packet = self.build(id.try_into().unwrap(),host);
        self.socket_send(&packet);
    }

    pub fn move_out_dns_result(&mut self) -> Vec<(u64,Option<String>)> {
        //self.log(format!("move_out_dns_result {:?}",self.dns_result));
        let ret = self.dns_result.clone();
        self.clear_dns_result();
        ret
    }

    pub fn clear_dns_result(&mut self) {
        //self.log(format!("clear_dns_result {:?}",self.dns_result));
        self.dns_result.clear();
    }
    
}

impl Line {
    fn build(&self,id:u16,host:String) -> Vec<u8> {
        let mut packet = Packet::new_query(id);
        packet.set_flags(PacketFlag::RECURSION_DESIRED);
        let qname = Name::new(&host).unwrap();
        let qtype = TYPE::A.into();
        let qclass = CLASS::IN.into();
        let question = Question::new(qname, qtype, qclass, false);
        packet.questions.push(question);
        packet.build_bytes_vec_compressed().unwrap()
    }

    fn decode(&mut self,buf:&[u8]) -> (u64,Option<String>) {
        match Packet::parse(buf) {
            Ok(packet) => {
                let id = packet.id() as u64;
                match packet.rcode() {
                    RCODE::NoError => {
                        let ip = get_ip(packet.answers);
                        (id,ip)
                    },
                    other => {
                        self.log(format!("dns server reply with error code {:?}",other));
                        (id,None)
                    },
                }
            },
            Err(e) => {
                self.log(format!("packet parse fail {},{}",e,buf.len()));
                (0,None)
            },
        }
    }

    
}

fn get_ip(data:Vec<ResourceRecord>) -> Option<String> {
    for v in data {
        match v.rdata {
            rdata::RData::A(a) => {
                let b = a.address.to_be_bytes();
                let ret = format!("{}.{}.{}.{}",b[0],b[1],b[2],b[3]);
                return Some(ret)
            }
            _ => {}
        }
    }

    None
}