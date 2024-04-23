use crate::ss::Line;

impl Line {
    pub fn set_website_host(&mut self,s:String) {
        self.log(format!("website_host change from {:?} to {:?}",self.website_host,s));
        self.website_host = s;
    }
}
