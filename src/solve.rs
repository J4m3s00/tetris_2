use crate::{board::Board, piece::Piece, Position};

pub enum SolveResult {
    NoMorePieces,
    NotSolvable(Board),
    Solved(Board),
}

trait Solvable {
    fn solve(&self, board: &Board, pieces: &[Piece]) -> SolveResult;
}

pub mod dumb_solver;

pub fn solve<S: Solvable>(solver: S, board: &Board, pieces: &[Piece]) -> SolveResult {
    solver.solve(board, pieces)
}
