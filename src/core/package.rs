use legion::{Resources, World};

use crate::ScionBuilder;

pub trait Package {
    fn load(self, builder: ScionBuilder) -> ScionBuilder;

    fn prepare(&self, _world: &mut World, _resources: &mut Resources) {}
}
