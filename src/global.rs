use chrono::Local;

static mut FRAME: i32 = 0;

static mut ID: u64 = 10;

pub fn frame() -> i32 {
    unsafe { FRAME }
}

pub fn next_frame() {
    unsafe { FRAME = FRAME + 1 };
}

pub fn next_id() -> u64 {
    unsafe { 
        ID = ID + 1;
        return ID; 
    };
}

pub fn u8r(input:u8) -> u8 {
    if input > 45 && input < 255 - 45 {
        255 - input
    } else {
        input
    }
}

pub fn reverse(buf:&mut[u8]) {
    for i in 0..buf.len() {
        buf[i] = u8r(buf[i]);
    }
}

pub fn now() -> i64 {
    let now = Local::now();
    now.timestamp()
}

