use gate::Gate;

#[cfg_attr(feature  = "hongkong", path = "hongkong.rs")]
mod default_config;

mod config;
mod global;
mod log;
mod gate;
mod line;

fn main() {
    log::init();
    Gate::new().start();
}