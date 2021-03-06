use scion::application::Scion;
use scion::legion::system;

#[system]
fn hello(){
    println!("Hello from system")
}

fn main() {
    Scion::app()
        .with_system(hello_system())
        .run();
}