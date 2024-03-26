use crate::{board::Board, piece::Piece};

pub enum SolveResult {
    ToManyTries,
    NoMorePieces,
    NotSolvable(Board),
    Solved(Board),
}

pub trait Solvable {
    fn solve(&self, board: &Board, pieces: &[Piece]) -> SolveResult;
}

pub mod dumb_solver;

pub fn solve<S: Solvable>(solver: S, board: &Board, pieces: &[Piece]) -> SolveResult {
    solver.solve(board, pieces)
}
