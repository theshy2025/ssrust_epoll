use std::{fs::{self, File, OpenOptions}, io::Write};

use crate::{config::{self}, global::frame};

use self::buf_writer::LogBufWriter;

pub mod buf_writer;
pub mod log_dir;

pub trait Log {
    fn id(&self) -> u64;

    fn logger(&mut self) -> &mut LogBufWriter;

    fn log(&mut self,s:String) {
        self.logger().add(s);
        self.logger().flush();
    }
}

pub fn init() {
    let device = config::device();
    match fs::remove_dir_all( &device ) {
        Ok(_) => {}
        Err(_) => {},
    }
    fs::create_dir_all( &device ).unwrap();
    File::create( format!("{}/.log",&device) ).unwrap();
    File::create( format!("{}/err.log",&device) ).unwrap();
}



pub fn im(s:String) {
    let device = config::device();
    write(format!("[{}]{}\n",frame(),s),format!("{}/default.log",device));
}

pub fn err(s:String) {
    let device = config::device();
    write(format!("[{}]{}\n",frame(),s),format!("{}/err.log",device));
}

fn write(s:String,path:String) {
    match OpenOptions::new().append(true).open( &path ) {
        Ok(mut f) => {
            f.write(s.as_bytes()).unwrap();
        },
        Err(e) => println!("{:?},{:?}",e,path)
    }
}


