use scion::Scion;
use scion::legion::{system, World, Resources};
use scion::game_layer::{SimpleGameLayer, GameLayer, GameLayerController};

#[system]
fn test(){
    log::info!("Hello all");
}

#[derive(Default)]
struct LayerA{
    tmp: usize
}

impl SimpleGameLayer for LayerA{
    fn update(&mut self, _world: &mut World, resource: &mut Resources) {
        log::info!("HeullohA...{}", self.tmp);
        self.tmp += 1;
        if self.tmp >= 301 {
            resource.get_mut::<GameLayerController>().unwrap().pop_layer();
            resource.get_mut::<GameLayerController>().unwrap().push_layer(GameLayer::strong::<LayerB>());
        }
    }
}

#[derive(Default)]
struct LayerB;

impl SimpleGameLayer for LayerB{
    fn update(&mut self, _world: &mut World, _resource: &mut Resources) {
        log::info!("HeullohB...");
    }
}

fn main() {
    Scion::app()
        .with_game_layer(
            GameLayer::weak::<LayerA>()
        )
        .run();
}