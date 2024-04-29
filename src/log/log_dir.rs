use std::fs;

use crate::config::{self};

use super::buf_writer::LogBufWriter;

pub trait LogDir {
    fn dir_name() -> String {
        let ret = std::any::type_name::<Self>();
        let name = ret.split("::").last().unwrap();
        name.to_string()
    }

    fn create_dir() {
        let device = config::device();
        let dir = Self::dir_name();
        let path = format!("{}/{}",device,dir);
        fs::create_dir_all( path ).unwrap();
    }

    fn create_buf_writer(id:u64) -> LogBufWriter {
        let device = config::device();
        let dir = Self::dir_name();
        let path = format!("{}/{}/{}.log",device,dir,id);
        LogBufWriter::new(path).unwrap()
    }
}