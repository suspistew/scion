use scion::application::Scion;
use scion::legion::system;
use scion::utils::time::Time;
use log::info;


#[system]
fn hello(#[resource] time: &Time) {
    info!("Heulloh elapsed : {:?}", time.delta_duration());
}

fn main() {
    Scion::app().with_system(hello_system()).run();
}
