/// `GameState` is a convenience Resource created to keep track of
/// diverse thing internally. It's also the resource used to create
/// pausable systems.
#[derive(Debug, Copy, Clone, Default)]
pub struct GameState {}

impl GameState {
    pub fn test(&self) -> bool {
        true
    }
}
