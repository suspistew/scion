use crate::components::Piece;

pub enum TetrisState {
    MOVING(u32, u32),
    WAITING,
}

pub struct TetrisResource{
    pub state: TetrisState,
    pub active_piece: Piece,
    pub next_piece: Piece,
    pub score: usize
}

impl Default for TetrisResource{
    fn default() -> Self {
        Self{
            state: TetrisState::WAITING,
            active_piece: Piece::random_new(),
            next_piece: Piece::random_new(),
            score: 0
        }
    }
}

impl TetrisResource {
    pub fn switch_to_next_piece(&mut self) {
        self.state = TetrisState::MOVING(4, 0);
        self.active_piece = self.next_piece.clone(); // TODO : Is there something I can do about this ?
        self.next_piece = Piece::random_new();
    }
}