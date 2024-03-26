use crate::{board::Board, piece::Piece, Position};

use super::{Solvable, SolveResult};

pub struct DumbSolver;

impl Solvable for DumbSolver {
    fn solve(
        &self,
        board: &crate::board::Board,
        pieces: &[Vec<crate::piece::Piece>],
    ) -> super::SolveResult {
        step(board, pieces, 0)
    }
}

pub fn step(board: &Board, pieces: &[Vec<Piece>], depth: u16) -> SolveResult {
    if pieces.is_empty() {
        return SolveResult::NoMorePieces;
    }
    let all_transforms = &pieces[0];

    for y in 0..8 {
        for x in 0..8 {
            // Try piece on every position and rotation/flipped
            let pos = Position::new(x, y);
            for piece in all_transforms.iter() {
                if board.can_place_piece(pos, piece) {
                    let mut board_clone = board.clone();
                    board_clone.place_piece(pos, piece);
                    if has_single_cells(&board_clone) {
                        continue;
                    }
                    if depth == 3 {
                        println!("{board_clone}");
                    }
                    if board_clone.is_solved() {
                        return SolveResult::Solved(board_clone.clone());
                    }
                    if let SolveResult::Solved(finished) =
                        step(&board_clone, &pieces[1..], depth + 1)
                    {
                        return SolveResult::Solved(finished);
                    }
                }
            }
        }
    }
    SolveResult::NotSolvable
}

fn has_neighbour(board: &Board, position: &Position, direction: (i8, i8)) -> bool {
    let (n_x, n_y) = position.offset(direction);
    if n_x < 0 || n_x > 7 {
        true
    } else if n_y < 0 || n_y > 7 {
        true
    } else {
        board.get_value(Position::new(n_x as u8, n_y as u8)) != 0
    }
}

fn has_single_cells(board: &Board) -> bool {
    (0..8)
        .map(|y| (0..8).map(move |x| (x, y)))
        .flatten()
        .any(|(x, y)| {
            let position = Position::new(x, y);
            if board.get_value(position) != 0 {
                return false;
            }
            // If the cell has on all sides neightbours, we can discard that board
            [(-1, 0), (0, -1), (1, 0), (0, 1)]
                .iter()
                .all(|direction| has_neighbour(board, &position, *direction))
        })
}
