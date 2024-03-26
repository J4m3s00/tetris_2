use crate::{board::Board, piece::Piece};

/// The coords will always be between 0 and 7
#[derive(Clone, Copy, Debug)]
pub struct Position {
    x: u8,
    y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        assert!(x < 8);
        assert!(y < 8);

        Self { x, y }
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
        self.y
    }

    pub fn try_add(&self, other: &Self) -> Option<Self> {
        let x = self.x + other.x;
        let y = self.y + other.y;
        if x < 8 && y < 8 {
            Some(Position { x, y })
        } else {
            None
        }
    }
}

mod board;
mod piece;
mod solve;

fn main() {
    let test_piece = Piece::new(
        1,
        vec![
            Position::new(1, 0),
            Position::new(0, 1),
            Position::new(1, 1),
            Position::new(1, 2),
            Position::new(2, 2),
        ],
    );
    let mut board = Board::default();
    board.place_piece(Position::new(0, 0), &test_piece);
    board.place_piece(Position::new(0, 4), &test_piece.rotate_ccw());
    board.place_piece(Position::new(4, 0), &test_piece.rotate_180());
    board.place_piece(Position::new(4, 4), &test_piece.rotate_cw());

    println!("{board}");
}

fn get_game_pieces() -> Vec<Piece> {
    vec![
        Piece::new(
            1,
            vec![
                Position::new(1, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(1, 2),
                Position::new(2, 2),
            ],
        ),
        Piece::new(
            2,
            vec![
                Position::new(2, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
                Position::new(0, 2),
            ],
        ),
        Piece::new(
            3,
            vec![
                Position::new(0, 0),
                Position::new(2, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
                Position::new(0, 2),
                Position::new(2, 2),
            ],
        ),
        Piece::new(
            4,
            vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(1, 2),
                Position::new(2, 2),
            ],
        ),
        Piece::new(
            5,
            vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(2, 0),
                Position::new(0, 1),
                Position::new(2, 1),
            ],
        ),
        Piece::new(
            6,
            vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(2, 1),
                Position::new(3, 1),
            ],
        ),
        Piece::new(
            7,
            vec![
                Position::new(1, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
            ],
        ),
        Piece::new(
            8,
            vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(0, 1),
                Position::new(1, 1),
            ],
        ),
        Piece::new(
            9,
            vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(2, 1),
            ],
        ),
        Piece::new(
            10,
            vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(2, 0),
                Position::new(3, 0),
                Position::new(1, 1),
            ],
        ),
        Piece::new(
            11,
            vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(2, 0),
                Position::new(3, 0),
                Position::new(0, 1),
            ],
        ),
        Piece::new(
            12,
            vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(2, 0),
                Position::new(0, 1),
            ],
        ),
        Piece::new(
            13,
            vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(2, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
            ],
        ),
    ]
}
