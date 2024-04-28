use std::{fs::{self, File, OpenOptions}, io::Write};

use crate::{default_config::DEVICE, global::frame};

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
    match fs::remove_dir_all( DEVICE ) {
        Ok(_) => {}
        Err(_) => {},
    }
    fs::create_dir_all( DEVICE ).unwrap();
    File::create( format!("{}/.log",DEVICE) ).unwrap();
    File::create( format!("{}/err.log",DEVICE) ).unwrap();
}



pub fn im(s:String) {
    write(format!("[{}]{}\n",frame(),s),format!("{}/.log",DEVICE));
}

pub fn err(s:String) {
    write(format!("[{}]{}\n",frame(),s),format!("{}/err.log",DEVICE));
}

fn write(s:String,path:String) {
    match OpenOptions::new().append(true).open( &path ) {
        Ok(mut f) => {
            f.write(s.as_bytes()).unwrap();
        },
        Err(e) => println!("{:?},{:?}",e,path)
    }
}


