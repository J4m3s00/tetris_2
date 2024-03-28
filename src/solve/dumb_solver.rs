use crate::{
    board::{Board, BoardID},
    piece::Piece,
    Position,
};

use super::{Solvable, SolveResult, SolveStats};

pub struct DumbSolver;

impl Solvable for DumbSolver {
    fn solve(
        &self,
        stats: &mut SolveStats,
        board: &crate::board::Board,
        pieces: &[Vec<crate::piece::Piece>],
    ) -> super::SolveResult {
        let root = stats.insert_board(None, board.clone());
        step(stats, board, pieces, 0, root)
    }
}

pub fn step(
    stats: &mut SolveStats,
    board: &Board,
    pieces: &[Vec<Piece>],
    depth: u16,
    parent: BoardID,
) -> SolveResult {
    if pieces.is_empty() {
        println!("Empty!");
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
                    let new_parent = stats.insert_board(Some(parent.clone()), board_clone);
                    if has_single_cells(&board_clone) {
                        stats.num_skiped_single += 1;
                        continue;
                    }
                    if depth == 4 {
                        println!("{board_clone}");
                    }
                    if board_clone.is_solved() {
                        return SolveResult::Solved(board_clone.clone());
                    }
                    if let SolveResult::Solved(finished) =
                        step(stats, &board_clone, &pieces[1..], depth + 1, new_parent)
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
