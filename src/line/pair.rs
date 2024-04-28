use super::status::LineStatus;

pub trait LinePair : LineStatus {
    fn pair_id(&self) -> u64 {
        0
    }

    fn set_pair_id(&mut self,_id:u64){}

    fn on_pair_close(&mut self) {
        self.log(format!("on_pair_close"));
        self.set_pair_id(0);
    }
}