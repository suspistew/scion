use scion::application::Scion;
use scion::legion::{system};
use scion::utils::time::Time;

#[system]
fn hello(#[resource] time: &Time){
    println!("Hello from system {:?}", time.delta_duration());
}

fn main() {
    Scion::app()
        .with_system(hello_system())
        .run();
}