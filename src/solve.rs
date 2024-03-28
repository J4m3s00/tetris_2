use std::fmt::Display;

use crate::{board::Board, piece::Piece};

pub mod dumb_solver;
mod empty_solver;

pub type SolveResult<T> = Result<Board, T>;

/// Trait for a solvable
/// Impl this type to invoke solve with it.
/// It should return a single solved board or an error if it is not solvable
pub trait Solvable {
    type SolveStats: Default + Display;
    type Failure: Display;

    fn solve(
        &self,
        stats: &mut Self::SolveStats,
        board: &Board,
        pieces: &[Vec<Piece>],
    ) -> SolveResult<Self::Failure>;
}

/// Solves the board with an provided Solvable
pub fn solve<S: Solvable>(
    solver: S,
    board: &Board,
    pieces: &[Vec<Piece>],
) -> (SolveResult<S::Failure>, S::SolveStats) {
    let mut stats = S::SolveStats::default();
    (solver.solve(&mut stats, board, pieces), stats)
}
