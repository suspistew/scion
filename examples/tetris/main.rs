use scion::Scion;
use scion::legion::system;
use std::path::Path;

#[system]
fn test(){
    log::info!("Hello all");
}

fn main() {
    Scion::app_with_config_path(&Path::new("/Users/jeremythulliez/Desktop/scion.json"))
        .with_pausable_system(test_system(), |state| state.test())
        .run();
}