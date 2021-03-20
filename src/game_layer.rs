//! Everything that is linked to the running of game layers.
use legion::{Resources, World};

/// Trait to implement in order to define a `GameLayer`.
pub trait SimpleGameLayer {
    /// Will be called once before the new game loop iteration. Useful to initialize resources and add everything you need in the world.
    fn on_start(&mut self, _world: &mut World, _resource: &mut Resources) {}
    /// Will be called each game loop, before the systems execution
    fn update(&mut self, _world: &mut World, _resource: &mut Resources) {}
    /// Will be called each game loop, after the systems execution
    fn late_update(&mut self, _world: &mut World, _resource: &mut Resources) {}
    /// Actually not used right now, but will in the future
    fn on_stop(&mut self, _world: &mut World, _resource: &mut Resources) {}
}

pub(crate) enum LayerAction {
    Update,
    Start,
    _STOP,
    LateUpdate,
}

/// A game layer is one line in the pile of game layers.
/// Each layer will be executed from the top of the pile until the end or a strong layer.
pub struct GameLayer {
    pub(crate) layer: GameLayerType,
}

impl GameLayer {
    /// Creates a weak layer of type T. A weak layer won't block the pile's next layer execution.
    pub fn weak<T: SimpleGameLayer + Default + 'static>() -> Box<GameLayer> {
        Box::new(GameLayer {
            layer: GameLayerType::Weak(Box::new(T::default())),
        })
    }

    /// Creates a strong layer of type T. A strong layer will be the last executed in the pile.
    pub fn strong<T: SimpleGameLayer + Default + 'static>() -> Box<GameLayer> {
        Box::new(GameLayer {
            layer: GameLayerType::Strong(Box::new(T::default())),
        })
    }
}

/// The `GameLayer` determines if the layer is blocking the execution queue.
pub(crate) enum GameLayerType {
    /// All the layers that are under the `STRONG` Layers won't be executed.
    Strong(Box<dyn SimpleGameLayer>),
    /// The next layer under a `WEAK` layer will be executed
    Weak(Box<dyn SimpleGameLayer>),
}
