use rand::{thread_rng, Rng};

pub const BLOC_SIZE: f32 = 32.0;

#[derive(Debug)]
pub enum BlocKind {
    Moving,
    Static,
}

#[derive(Debug)]
pub struct Bloc {
    pub kind: BlocKind,
}

pub struct NextBloc;

impl Bloc {
    pub fn new(k: BlocKind) -> Bloc {
        Bloc { kind: k }
    }
}

#[derive(Debug, Clone)]
pub enum PieceOrientation {
    Up,
    Right,
    Down,
    Left,
}

impl PieceOrientation {
    pub fn next_orientation(&self) -> PieceOrientation {
        match self {
            PieceOrientation::Up => PieceOrientation::Right,
            PieceOrientation::Right => PieceOrientation::Down,
            PieceOrientation::Down => PieceOrientation::Left,
            PieceOrientation::Left => PieceOrientation::Up,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum PieceKind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl PieceKind {
    pub fn from_int(x: u8) -> Result<PieceKind, &'static str> {
        match x {
            x if x == PieceKind::I as u8 => Ok(PieceKind::I),
            x if x == PieceKind::O as u8 => Ok(PieceKind::O),
            x if x == PieceKind::T as u8 => Ok(PieceKind::T),
            x if x == PieceKind::S as u8 => Ok(PieceKind::S),
            x if x == PieceKind::Z as u8 => Ok(PieceKind::Z),
            x if x == PieceKind::J as u8 => Ok(PieceKind::J),
            x if x == PieceKind::L as u8 => Ok(PieceKind::L),
            _ => Err("Error while convertine u8 to PieceKind"),
        }
    }

    pub fn get_self_offsets(&self, orientation: &PieceOrientation) -> Vec<(f32, f32)> {
        PieceKind::get_offsets(self, orientation)
    }

    pub fn get_offsets(kind: &PieceKind, orientation: &PieceOrientation) -> Vec<(f32, f32)> {
        match kind {
            PieceKind::I => PieceKind::get_i_offsets(orientation),
            PieceKind::O => vec![(1.0, 1.0), (2.0, 1.0), (1.0, 2.0), (2.0, 2.0)],
            PieceKind::T => PieceKind::get_t_offsets(orientation),
            PieceKind::S => PieceKind::get_s_offsets(orientation),
            PieceKind::Z => PieceKind::get_z_offsets(orientation),
            PieceKind::J => PieceKind::get_j_offsets(orientation),
            PieceKind::L => PieceKind::get_l_offsets(orientation),
        }
    }

    fn get_i_offsets(orientation: &PieceOrientation) -> Vec<(f32, f32)> {
        match orientation {
            PieceOrientation::Up => vec![(0.0, 1.0), (1.0, 1.0), (2.0, 1.0), (3.0, 1.0)],
            PieceOrientation::Right => vec![(2.0, 0.0), (2.0, 1.0), (2.0, 2.0), (2.0, 3.0)],
            PieceOrientation::Down => vec![(0.0, 2.0), (1.0, 2.0), (2.0, 2.0), (3.0, 2.0)],
            PieceOrientation::Left => vec![(1.0, 0.0), (1.0, 1.0), (1.0, 2.0), (1.0, 3.0)],
        }
    }

    fn get_t_offsets(orientation: &PieceOrientation) -> Vec<(f32, f32)> {
        match orientation {
            PieceOrientation::Up => vec![(1.0, 0.0), (0.0, 1.0), (1.0, 1.0), (2.0, 1.0)],
            PieceOrientation::Right => vec![(1.0, 0.0), (1.0, 1.0), (2.0, 1.0), (1.0, 2.0)],
            PieceOrientation::Down => vec![(0.0, 1.0), (1.0, 1.0), (2.0, 1.0), (1.0, 2.0)],
            PieceOrientation::Left => vec![(1.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 2.0)],
        }
    }

    fn get_s_offsets(orientation: &PieceOrientation) -> Vec<(f32, f32)> {
        match orientation {
            PieceOrientation::Up => vec![(1.0, 0.0), (2.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
            PieceOrientation::Right => vec![(1.0, 0.0), (1.0, 1.0), (2.0, 1.0), (2.0, 2.0)],
            PieceOrientation::Down => vec![(1.0, 1.0), (2.0, 1.0), (0.0, 2.0), (1.0, 2.0)],
            PieceOrientation::Left => vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 2.0)],
        }
    }

    fn get_z_offsets(orientation: &PieceOrientation) -> Vec<(f32, f32)> {
        match orientation {
            PieceOrientation::Up => vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (2.0, 1.0)],
            PieceOrientation::Right => vec![(2.0, 0.0), (1.0, 1.0), (2.0, 1.0), (1.0, 2.0)],
            PieceOrientation::Down => vec![(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (2.0, 2.0)],
            PieceOrientation::Left => vec![(1.0, 0.0), (0.0, 1.0), (1.0, 1.0), (0.0, 2.0)],
        }
    }

    fn get_j_offsets(orientation: &PieceOrientation) -> Vec<(f32, f32)> {
        match orientation {
            PieceOrientation::Up => vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (2.0, 1.0)],
            PieceOrientation::Right => vec![(1.0, 0.0), (2.0, 0.0), (1.0, 1.0), (1.0, 2.0)],
            PieceOrientation::Down => vec![(0.0, 1.0), (1.0, 1.0), (2.0, 1.0), (2.0, 2.0)],
            PieceOrientation::Left => vec![(1.0, 0.0), (1.0, 1.0), (0.0, 2.0), (1.0, 2.0)],
        }
    }

    fn get_l_offsets(orientation: &PieceOrientation) -> Vec<(f32, f32)> {
        match orientation {
            PieceOrientation::Up => vec![(2.0, 0.0), (0.0, 1.0), (1.0, 1.0), (2.0, 1.0)],
            PieceOrientation::Right => vec![(1.0, 0.0), (1.0, 1.0), (1.0, 2.0), (2.0, 2.0)],
            PieceOrientation::Down => vec![(0.0, 1.0), (1.0, 1.0), (2.0, 1.0), (0.0, 2.0)],
            PieceOrientation::Left => vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (1.0, 2.0)],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub orientation: PieceOrientation,
    pub kind: PieceKind,
    pub color: usize,
}

impl Piece {
    pub fn new(o: PieceOrientation, k: PieceKind, c: usize) -> Piece {
        Piece {
            orientation: o,
            kind: k,
            color: c,
        }
    }

    pub fn get_current_offsets(&self) -> Vec<(f32, f32)> {
        self.kind.get_self_offsets(&self.orientation)
    }

    pub fn rotate(&mut self) {
        self.orientation = self.orientation.next_orientation();
    }

    pub fn random_new() -> Piece {
        let random_piece_nb: u8 = thread_rng().gen_range(0..7);
        let piece_kind = PieceKind::from_int(random_piece_nb).unwrap();
        Piece {
            orientation: PieceOrientation::Right,
            kind: piece_kind,
            color: thread_rng().gen_range(0..7),
        }
    }
}

pub const BOARD_HEIGHT: u32 = 20;
pub const BOARD_WIDTH: u32 = 10;
pub const BOARD_OFFSET: (f32, f32) = (32., 32.);
