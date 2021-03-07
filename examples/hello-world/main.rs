use scion::application::Scion;
use scion::legion::system;
use scion::utils::time::Time;

#[system]
fn hello(#[resource] _time: &Time) {}

fn main() {
    Scion::app().with_system(hello_system()).run();
}
