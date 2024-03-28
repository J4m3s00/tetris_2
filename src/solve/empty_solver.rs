use std::fmt::Display;

use super::Solvable;

struct EmptySolver;

#[derive(Default)]
struct EmptyType;

impl Display for EmptyType {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Solvable for EmptySolver {
    type SolveStats = EmptyType;

    type Failure = EmptyType;

    fn solve(
        &self,
        _: &mut Self::SolveStats,
        board: &crate::board::Board,
        _: &[Vec<crate::piece::Piece>],
    ) -> super::SolveResult<Self::Failure> {
        Ok(*board)
    }
}
