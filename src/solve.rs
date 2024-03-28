use std::fmt::Display;

use crate::{board::Board, piece::Piece};

pub mod dumb_solver;
mod empty_solver;

pub type SolveResult<T> = Result<Board, T>;

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

pub fn solve<S: Solvable>(
    solver: S,
    board: &Board,
    pieces: &[Vec<Piece>],
) -> (SolveResult<S::Failure>, S::SolveStats) {
    let mut stats = S::SolveStats::default();
    (solver.solve(&mut stats, board, pieces), stats)
}
