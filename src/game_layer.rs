use legion::{Resources, World};


/// Trait to implement in order to create a `GameLayer`.
pub trait SimpleGameLayer {
    fn on_start(&mut self, _world: &mut World, _resource: &mut Resources) {}
    fn update(&mut self, _world: &mut World, _resource: &mut Resources) {}
    fn late_update(&mut self, _world: &mut World, _resource: &mut Resources) {}
    fn on_stop(&mut self, _world: &mut World, _resource: &mut Resources) {}
}

pub(crate) enum LayerAction {
    UPDATE, START, STOP, LATE_UPDATE
}

pub struct GameLayer{
    pub(crate) layer: GameLayerType
}

impl GameLayer {
    pub fn weak<T: SimpleGameLayer + Default + 'static>() -> Box<GameLayer> {
        Box::new(GameLayer{layer: GameLayerType::Weak(Box::new(T::default()))})
    }

    pub fn strong<T: SimpleGameLayer + Default + 'static>() -> Box<GameLayer> {
        Box::new(GameLayer{layer: GameLayerType::Strong(Box::new(T::default()))})
    }
}

/// The `GameLayer` determines if the layer is blocking the execution queue.
pub enum GameLayerType {
    /// All the layers that are under the `STRONG` Layers won't be executed.
    Strong(Box<dyn SimpleGameLayer>),
    /// The next layer under a `WEAK` layer will be executed
    Weak(Box<dyn SimpleGameLayer>),
}