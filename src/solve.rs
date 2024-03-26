use crate::{board::Board, piece::Piece};

pub enum SolveResult {
    NoMorePieces,
    NotSolvable,
    Solved(Board),
}

pub trait Solvable {
    fn solve(&self, board: &Board, pieces: &[Vec<Piece>]) -> SolveResult;
}

pub mod dumb_solver;

pub fn solve<S: Solvable>(solver: S, board: &Board, pieces: &[Vec<Piece>]) -> SolveResult {
    solver.solve(board, pieces)
}
