use crate::core::world::World;
use crate::ScionBuilder;

pub trait Package {
    fn prepare(&self, _world: &mut World) {}

    fn load(self, builder: ScionBuilder) -> ScionBuilder;

}
