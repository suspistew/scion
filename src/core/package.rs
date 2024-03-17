use crate::core::world::GameData;
use crate::ScionBuilder;

pub trait Package {
    fn prepare(&self, _data: &mut GameData) {}

    fn load(&self, builder: ScionBuilder) -> ScionBuilder { builder }
}
