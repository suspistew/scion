//! Everything that is linked to the running of game layers.
use std::collections::VecDeque;

use legion::{Resources, World};

/// Trait to implement in order to define a `GameLayer`.
pub trait SimpleGameLayer {
    /// Will be called once before the new game loop iteration. Useful to initialize resources and add everything you need in the world.
    fn on_start(&mut self, _world: &mut World, _resource: &mut Resources) {}
    /// Will be called each game loop, before the systems execution
    fn update(&mut self, _world: &mut World, _resource: &mut Resources) {}
    /// Will be called each game loop, after the systems execution
    fn late_update(&mut self, _world: &mut World, _resource: &mut Resources) {}
    /// Will be called for deleted layer at the end of the frame where it was deleted
    fn on_stop(&mut self, _world: &mut World, _resource: &mut Resources) {}
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
                let current_layer = self
                    .game_layers
                    .get_mut(layer_index)
                    .expect("We just checked the len");
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
                            let layer = self.game_layers.pop();
                            if let Some(layer) = layer {
                                match layer.layer {
                                    GameLayerType::Strong(mut simple_layer)
                                    | GameLayerType::Weak(mut simple_layer) => {
                                        simple_layer.on_stop(world, resources);
                                    }
                                }
                            }
                        }
                        GameLayerTrans::Push(new_layer) => {
                            self.game_layers.push(new_layer);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub(crate) enum GameLayerTrans {
    Pop,
    Push(Box<GameLayer>),
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
    /// layer's stop will be deleted at the end of the frame.
    pub fn pop_layer(&mut self) {
        self.actions.push(GameLayerTrans::Pop);
    }
}

pub enum Error {
    EmptyLayersError,
}
