use std::{fs::{self, File, OpenOptions}, io::{BufWriter, Write}};

use chrono::{Local, Timelike};

use crate::{default_config::DEVICE, ss::Tag};

static mut FRAME: usize = 0;

pub struct Log { 
    w:BufWriter<File>
}

impl Log {
    pub fn new(path:String) -> Option<Log> {
        match File::create(&path) {
            Ok(f) => {
                let w = BufWriter::new(f);
                Some(Log { w })
            },
            Err(e) => {
                println!("{} {:?}",e,path);
                None
            },
        }
    }

    pub fn create_for_line(tag:&Tag,id:u64) -> Log {
        let path = format!("{}/{:?}/{}.log",DEVICE,tag,id);
        Log::new(path).unwrap()
    }

    pub fn create(name:&str) -> Log {
        let path = format!("{}/{}.log",DEVICE,name);
        Log::new(path).unwrap()
    }
}

impl Log {
    pub fn add(&mut self,s:String) {
        let now = Local::now();
        let t = format!("[{}:{:02}:{:02}:{}]",now.hour(),now.minute(),now.second(),now.timestamp_subsec_nanos());
        let s = format!("[{}]{}{}",frame(),t,s);
        writeln!(self.w,"{s}").unwrap();
    }

    pub fn flush(&mut self) {
        self.w.flush().unwrap();
    }
}



pub fn init() {
    match fs::remove_dir_all( DEVICE ) {
        Ok(_) => {}
        Err(_) => {},
    }
    fs::create_dir_all( DEVICE ).unwrap();
    File::create( format!("{}/.log",DEVICE) ).unwrap();
}

pub fn create_dir(tag:Tag) {
    let path = format!("{}/{:?}",DEVICE,tag);
    fs::create_dir_all( path ).unwrap();
}

pub fn im(s:String) {
    write(format!("[{}]{}\n",frame(),s),format!("{}/.log",DEVICE));
}

fn write(s:String,path:String) {
    match OpenOptions::new().append(true).open( &path ) {
        Ok(mut f) => {
            f.write(s.as_bytes()).unwrap();
        },
        Err(e) => println!("{:?},{:?}",e,path)
    }
}

pub fn frame() -> usize {
    unsafe { FRAME }
}

pub fn next_frame() {
    unsafe { FRAME = FRAME + 1 };
}

pub fn now() -> i64 {
    let now = Local::now();
    now.timestamp()
}