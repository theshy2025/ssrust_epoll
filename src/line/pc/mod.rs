use socket2::Socket;

use crate::log::log_dir::LogDir;

use self::network::Step;

use super::base_line::BaseLine;

mod trait_impl;
mod network;

pub struct LinePc {
    pub basic:BaseLine,
    pub pair_id:u64,
    pub step:Step,
}

impl LinePc {
    pub fn new(id:u64,pair_id:u64,socket:Socket) -> LinePc {
        let buf_writer = LinePc::create_buf_writer(id);
        let basic = BaseLine::new(id, socket, buf_writer);
        LinePc { basic, pair_id, step: Step::Raw }
    }
}

