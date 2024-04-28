use std::fs;

use crate::default_config::DEVICE;

use super::buf_writer::LogBufWriter;

pub trait LogDir {
    fn dir_name() -> String {
        let ret = std::any::type_name::<Self>();
        let name = ret.split("::").last().unwrap();
        name.to_string()
    }

    fn create_dir() {
        let dir = Self::dir_name();
        let path = format!("{}/{}",DEVICE,dir);
        fs::create_dir_all( path ).unwrap();
    }

    fn create_buf_writer(id:u64) -> LogBufWriter {
        let dir = Self::dir_name();
        let path = format!("{}/{}/{}.log",DEVICE,dir,id);
        LogBufWriter::new(path).unwrap()
    }
}