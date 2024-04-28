use crate::log::Log;

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum Status {
    Raw,
    WriteOpen,
    Register,
    Establish,
    CoolDown,
    ReadClose,
    WriteClose,
    ReadWriteBothClose,
    DeRegister,
    Close,
    Dead,
}

pub trait LineStatus : Log {
    fn status(&self) -> Status {
        Status::Raw
    }

    fn set_status(&mut self,_new:Status){}
}