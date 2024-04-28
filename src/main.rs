use gate::Gate;

#[cfg_attr(feature  = "hongkong", path = "hongkong.rs")]
mod default_config;

mod config;
mod log;
mod gate;
mod line;
mod global;

fn main() {
    log::init();
    Gate::new().start();
}

/*

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum  Status {
    Raw,
    WriteOpen,
    Register,
    Established,
    WaitingDnsResult,
    DnsQuerySuccess,
    WorldConnectSuccess,
    FirstDone,
    SecondDone,
    EncryptDone,
    CoolDown,
    Ready,
    ReadClose,
    WriteClose,
    ReadWriteBothClose,
    DeRegister,
    Close,
    Dead,
}

use crate::log::Log;


pub trait LineStatus: Log {
    fn status(&self) -> Status {
        Status::Raw
    }
    
    fn set_status(&mut self,_new:Status){}

    fn update_status(&mut self,new:Status) {
        let old = self.status();
        if old == new {
            return;
        }
        self.log(format!("change status from {:?} to {:?} ",old,new));
        
        self.set_status(new);
    }

    fn turn_dead(&mut self) {
        self.log(format!("turn_dead"));
        //self.set_pair_id(0);
        self.update_status(Status::Dead);
    }

}

*/