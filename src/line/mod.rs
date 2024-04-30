use std::any::Any;

use self::event::LineEvent;

pub mod base_line;
pub mod dns;
pub mod pc;
pub mod tcp2vps;
pub mod udp2vps;
pub mod mainland;
pub mod world;
pub mod status;
pub mod pair;
mod event;
mod network;



pub trait LineTrait : LineEvent {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

