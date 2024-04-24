use crate::ss::Line;

impl Line {
    pub fn set_peer_ip(&mut self,s:String) {
        self.log(format!("peer_ip change from {:?} to {:?}",self.peer_ip,s));
        self.peer_ip = s;
    }
}
