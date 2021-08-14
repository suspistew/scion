//! Everything that is linked to the running of game layers.
use std::collections::VecDeque;

use legion::{Resources, World};

/// Trait to implement in order to define a `GameLayer`.
pub trait SimpleGameLayer {
    /// Will be called once before the new game loop iteration. Useful to initialize resources and add everything you need in the world.
    fn on_start(&mut self, _world: &mut World, _resources: &mut Resources) {}
    /// Will be called each game loop, before the systems execution
    fn update(&mut self, _world: &mut World, _resources: &mut Resources) {}
    /// Will be called each game loop, after the systems execution
    fn late_update(&mut self, _world: &mut World, _resources: &mut Resources) {}
    /// Will be called for deleted layer at the end of the frame where it was deleted
    fn on_stop(&mut self, _world: &mut World, _resources: &mut Resources) {}
}

pub(crate) enum LayerAction {
    Update,
    Start,
    EndFrame,
    LateUpdate,
}

/// A game layer is one line in the pile of game layers.
/// Each layer will be executed from the top of the pile until the end or a strong layer.
pub struct GameLayer {
    pub(crate) name: String,
    pub(crate) layer: GameLayerType,
}

impl GameLayer {
    /// Creates a weak layer of type T. A weak layer won't block the pile's next layer execution.
    pub fn weak<T: SimpleGameLayer + Default + 'static>(name: &str) -> Box<GameLayer> {
        Box::new(GameLayer {
            name: name.to_string(),
            layer: GameLayerType::Weak(Box::new(T::default())),
        })
    }

    /// Creates a strong layer of type T. A strong layer will be the last executed in the pile.
    pub fn strong<T: SimpleGameLayer + Default + 'static>(name: &str) -> Box<GameLayer> {
        Box::new(GameLayer {
            name: name.to_string(),
            layer: GameLayerType::Strong(Box::new(T::default())),
        })
    }

    pub(crate) fn start(&mut self, world: &mut World, resources: &mut Resources) {
        let mut layer = &mut self.layer;

        match &mut layer {
            GameLayerType::Strong(simple_layer) | GameLayerType::Weak(simple_layer) => {
                simple_layer.on_start(world, resources);
            }
        };
    }
}

/// The `GameLayer` determines if the layer is blocking the execution queue.
pub(crate) enum GameLayerType {
    /// All the layers that are under the `STRONG` Layers won't be executed.
    Strong(Box<dyn SimpleGameLayer>),
    /// The next layer under a `WEAK` layer will be executed
    Weak(Box<dyn SimpleGameLayer>),
}

/// `GameLayerMachine` is the Resource used to control the game layers.
#[derive(Default)]
pub(crate) struct GameLayerMachine {
    pub(crate) game_layers: Vec<Box<GameLayer>>,
}

impl GameLayerMachine {
    pub(crate) fn apply_layers_action(
        &mut self,
        action: LayerAction,
        world: &mut World,
        resources: &mut Resources,
    ) {
        let layers_len = self.game_layers.len();
        if layers_len > 0 {
            for layer_index in (0..layers_len).rev() {
                let current_layer =
                    self.game_layers.get_mut(layer_index).expect("We just checked the len");
                match &mut current_layer.layer {
                    GameLayerType::Strong(simple_layer) | GameLayerType::Weak(simple_layer) => {
                        match action {
                            LayerAction::Update => simple_layer.update(world, resources),
                            LayerAction::Start => simple_layer.on_start(world, resources),
                            LayerAction::EndFrame => {
                                // Stop will only be called on dirty layers
                            }
                            LayerAction::LateUpdate => simple_layer.late_update(world, resources),
                        };
                    }
                }
                if let GameLayerType::Strong(_) = current_layer.layer {
                    break;
                }
            }
        }

        match action {
            LayerAction::EndFrame => {
                let mut game_layer_controller_actions: VecDeque<GameLayerTrans> = resources
                    .get_mut::<GameLayerController>()
                    .expect("Missing mandatory ressource : GameLayerController")
                    .actions
                    .drain(0..)
                    .collect();
                while let Some(layer_action) = game_layer_controller_actions.pop_front() {
                    match layer_action {
                        GameLayerTrans::Pop => {
                            if let Some(layer) = self.game_layers.pop() {
                                GameLayerMachine::stop_layer(world, resources, layer);
                            }
                        }
                        GameLayerTrans::Push(new_layer) => {
                            self.game_layers.push(new_layer);
                            let index = self.game_layers.len() - 1;
                            self.game_layers
                                .get_mut(index)
                                .expect("A recently added layer can't be found")
                                .start(world, resources);
                        }
                        GameLayerTrans::Replace(name, new_layer) => {
                            if let Some((index, old_layer)) =
                                self.find_layer_and_index_to_replace(name)
                            {
                                self.game_layers.insert(index, new_layer);
                                GameLayerMachine::stop_layer(world, resources, old_layer);
                                self.game_layers
                                    .get_mut(index)
                                    .expect("A recently added layer can't be found")
                                    .start(world, resources);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn stop_layer(world: &mut World, resources: &mut Resources, layer: Box<GameLayer>) {
        match layer.layer {
            GameLayerType::Strong(mut simple_layer) | GameLayerType::Weak(mut simple_layer) => {
                simple_layer.on_stop(world, resources);
            }
        }
    }

    fn find_layer_and_index_to_replace(&mut self, name: String) -> Option<(usize, Box<GameLayer>)> {
        if let Some(index) = self.game_layers.iter().position(|layer| layer.name.eq(name.as_str()))
        {
            Some((index, self.game_layers.remove(index)))
        } else {
            None
        }
    }
}

pub(crate) enum GameLayerTrans {
    Pop,
    Push(Box<GameLayer>),
    Replace(String, Box<GameLayer>),
}

/// `GameLayerController` is the Resource used to control the game layers.
#[derive(Default)]
pub struct GameLayerController {
    /// Vec of layer action that has to be executed at the end of the frame
    pub(crate) actions: Vec<GameLayerTrans>,
}

impl GameLayerController {
    /// Push `game_layer` to the top of the pile, note that it will happen in
    /// the end of the frame
    pub fn push_layer(&mut self, game_layer: Box<GameLayer>) {
        self.actions.push(GameLayerTrans::Push(game_layer));
    }

    /// Pop the top pile layer from the pile. If no layer exists, won't do anything. Note that the
    /// layer's stop will happen at the end of the frame.
    pub fn pop_layer(&mut self) { self.actions.push(GameLayerTrans::Pop); }

    /// Replace the layer with the name `layer_to_replace` with the layer `new_game_layer`. (Useful for level switching).
    /// If no layer exists with this name, won't do anything. Note that the layer's stop will happen at the end of the frame.
    pub fn replace_layer(&mut self, layer_to_replace: &str, new_game_layer: Box<GameLayer>) {
        self.actions.push(GameLayerTrans::Replace(layer_to_replace.to_string(), new_game_layer));
    }
}

pub enum Error {
    EmptyLayersError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct A;
    #[derive(Default)]
    struct B;
    #[derive(Default)]
    struct C;
    #[derive(Default)]
    struct D;

    impl SimpleGameLayer for A {}
    impl SimpleGameLayer for B {}
    impl SimpleGameLayer for C {}
    impl SimpleGameLayer for D {}

    #[test]
    fn replace_layer_should_replace_at_same_index() {
        let mut world = World::default();
        let mut resources = Resources::default();
        resources.insert(GameLayerController::default());

        let game_layers =
            vec![GameLayer::weak::<A>("A"), GameLayer::weak::<B>("B"), GameLayer::weak::<C>("C")];
        let mut machine = GameLayerMachine { game_layers };

        assert_eq!(1, machine.game_layers.iter().position(|l| l.name.eq("B")).unwrap());

        resources
            .get_mut::<GameLayerController>()
            .expect("Missing mandatory ressource : GameLayerController")
            .replace_layer("B", GameLayer::weak::<D>("D"));
        machine.apply_layers_action(LayerAction::EndFrame, &mut world, &mut resources);

        assert_eq!(1, machine.game_layers.iter().position(|l| l.name.eq("D")).unwrap());
    }
}
