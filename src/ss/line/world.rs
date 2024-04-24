use std::time::Instant;

use crate::ss::Line;

impl Line {
    
    pub fn on_data_from_world(&mut self,buf:&mut [u8]) -> usize {
        let len = buf.len();
        self.traffic = self.traffic + len;
        let t = self.clock.elapsed().as_millis();
        if t > 1000 {
            let kb = self.traffic/1024;
            self.log(format!("on_data_from_world {} bytes {:?} {}kb",len,self.status,kb));
            self.clock = Instant::now();
            self.traffic = 0;
        }
        
        buf.len()
    }

}