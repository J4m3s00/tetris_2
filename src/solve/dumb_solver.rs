use std::fmt::Display;

use crate::{
    board::Board,
    board_tree::{BoardID, BoardTree},
    piece::Piece,
    Position,
};

use super::{Solvable, SolveResult};

#[derive(Default)]
pub struct DumbStats {
    tree: BoardTree,
    pub num_checked_boards: usize,
    pub num_skiped_single: usize,
}

impl DumbStats {
    pub fn insert_board(&mut self, parent: Option<BoardID>, board: Board) -> BoardID {
        self.num_checked_boards += 1;
        self.tree.insert(board, parent)
    }
}

impl Display for DumbStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Checked boards: {}", self.num_checked_boards)?;
        writeln!(f, "Skipped single fields: {}", self.num_skiped_single)
    }
}

pub enum DumbFailure {
    NoMorePieces,
    NotSolvable,
}

impl Display for DumbFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DumbFailure::NoMorePieces => writeln!(f, "No more pieces"),
            DumbFailure::NotSolvable => writeln!(f, "Not Solvable"),
        }
    }
}

pub struct DumbSolver;

impl Solvable for DumbSolver {
    type SolveStats = DumbStats;
    type Failure = DumbFailure;

    fn solve(
        &self,
        stats: &mut Self::SolveStats,
        board: &crate::board::Board,
        pieces: &[Vec<crate::piece::Piece>],
    ) -> super::SolveResult<Self::Failure> {
        let root = stats.insert_board(None, *board);
        step(stats, board, pieces, 0, root)
    }
}

pub fn step(
    stats: &mut DumbStats,
    board: &Board,
    pieces: &[Vec<Piece>],
    depth: u16,
    parent: BoardID,
) -> SolveResult<DumbFailure> {
    if pieces.is_empty() {
        println!("Empty!");
        return Err(DumbFailure::NoMorePieces);
    }
    let all_transforms = &pieces[0];

    for y in 0..8 {
        for x in 0..8 {
            // Try piece on every position and rotation/flipped
            let pos = Position::new(x, y);
            for piece in all_transforms.iter() {
                if board.can_place_piece(pos, piece) {
                    let mut board_clone = *board;
                    board_clone.place_piece(pos, piece);
                    let new_parent = stats.insert_board(Some(parent), board_clone);
                    if has_single_cells(&board_clone) {
                        stats.num_skiped_single += 1;
                        continue;
                    }
                    if depth == 4 {
                        println!("{board_clone}");
                    }
                    if board_clone.is_solved() {
                        return Ok(board_clone);
                    }
                    if let Ok(finished) =
                        step(stats, &board_clone, &pieces[1..], depth + 1, new_parent)
                    {
                        return Ok(finished);
                    }
                }
            }
        }
    }
    Err(DumbFailure::NotSolvable)
}

fn has_neighbour(board: &Board, position: &Position, direction: (i8, i8)) -> bool {
    let (n_x, n_y) = position.offset(direction);
    if !(0..=7).contains(&n_x) || !(0..=7).contains(&n_y) {
        true
    } else {
        board.get_value(Position::new(n_x as u8, n_y as u8)) != 0
    }
}

fn has_single_cells(board: &Board) -> bool {
    (0..8)
        .flat_map(|y| (0..8).map(move |x| (x, y)))
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
