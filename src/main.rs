use gate::Gate;

mod config;
mod global;
mod log;
mod gate;
mod line;

fn main() {
    log::init();
    Gate::new().start();
}
