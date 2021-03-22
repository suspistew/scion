use scion::Scion;
use scion::legion::system;

#[system]
fn test(){
    log::info!("Hello all");
}

fn main() {
    Scion::app()
        .with_pausable_system(test_system(), |state| state.test())
        .run();
}