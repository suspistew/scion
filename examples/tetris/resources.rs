use scion::{graphics::components::material::Material, core::resources::asset_manager::AssetRef};

use crate::components::Piece;

pub enum TetrisState {
    MOVING(i32, i32),
    WAITING,
}

pub struct TetrisResource {
    pub asset: Option<AssetRef<Material>>,
    pub state: TetrisState,
    pub active_piece: Piece,
    pub next_piece: Piece,
    pub score: usize,
}

impl Default for TetrisResource {
    fn default() -> Self {
        Self {
            state: TetrisState::WAITING,
            active_piece: Piece::random_new(),
            next_piece: Piece::random_new(),
            score: 0,
            asset: None,
        }
    }
}

impl TetrisResource {
    pub fn switch_to_next_piece(&mut self) {
        self.state = TetrisState::MOVING(4, 0);
        self.active_piece = self.next_piece.clone();
        self.next_piece = Piece::random_new();
    }

    pub fn get_score(&self) -> String { format!("{:05}", self.score) }
}
