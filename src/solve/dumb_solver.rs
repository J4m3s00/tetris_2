use crate::{board::Board, piece::Piece, Position};

use super::{Solvable, SolveResult};

pub struct DumbSolver;

impl Solvable for DumbSolver {
    fn solve(
        &self,
        board: &crate::board::Board,
        pieces: &[crate::piece::Piece],
    ) -> super::SolveResult {
        step(board, pieces, 100)
    }
}

pub fn step(board: &Board, pieces: &[Piece], possible_depth: u32) -> SolveResult {
    if possible_depth == 0 {
        return SolveResult::ToManyTries;
    }
    if pieces.is_empty() {
        return SolveResult::NoMorePieces;
    }
    let all_transforms = pieces[0].get_all_transforms();

    for y in 0..8 {
        for x in 0..8 {
            // Try piece on every position and rotation/flipped
            let pos = Position::new(x, y);
            for piece in all_transforms.iter() {
                if board.can_place_piece(pos, piece) {
                    let mut board_clone = board.clone();
                    board_clone.place_piece(pos, piece);
                    //println!("Checking board:\n{board_clone}");
                    if let SolveResult::Solved(finished) =
                        step(&board_clone, &pieces[1..], possible_depth - 1)
                    {
                        return SolveResult::Solved(finished);
                    }
                }
            }
        }
    }
    //println!("Not solvable:\n{board}");
    SolveResult::NotSolvable(board.clone())
}
