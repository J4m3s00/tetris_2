use solve::{dumb_solver::DumbSolver, solve};

use crate::{board::Board, piece::Piece, position::Position};

pub mod board;
pub mod board_tree;
mod piece;
pub mod position;
mod solve;

fn main() {
    let pieces = get_game_pieces()
        .iter()
        .map(|p| p.get_all_transforms())
        .collect::<Vec<_>>();
    let board = Board::default();

    //let mut rng = rand::thread_rng();
    //pieces.shuffle(&mut rng);

    let (solve_result, stats) = solve(DumbSolver, &board, &pieces);
    match solve_result {
        Ok(b) => println!("Board solved!\n{b}"),
        Err(f) => println!("{f}"),
    }

    println!("{stats}");
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
