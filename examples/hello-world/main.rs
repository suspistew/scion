use scion::application::Scion;
use scion::legion::{system};
use scion::utils::time::Time;
use log::error;

#[system]
fn hello(#[resource] time: &Time){
}

fn main() {
    Scion::app()
        .with_system(hello_system())
        .run();
}