use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hasher},
};

use crate::{
    board::{Board, BoardID},
    piece::Piece,
};

pub enum SolveResult {
    NoMorePieces,
    NotSolvable,
    Solved(Board),
}

#[derive(Default)]
struct BoardTree {
    children: HashMap<BoardID, Vec<BoardID>>,
    parent: HashMap<BoardID, BoardID>,
    entries: HashMap<BoardID, Board>,
}

impl BoardTree {
    fn insert(&mut self, board: Board, parent: Option<BoardID>) -> BoardID {
        let mut hasher = DefaultHasher::new();
        hasher.write(board.as_slice());
        let board_id = hasher.finish();

        self.entries.insert(board_id, board);

        if let Some(parent) = parent {
            self.parent.insert(board_id, parent);
            self.children.entry(parent).or_default().push(board_id);
        }

        board_id
    }

    fn _get_board(&self, id: BoardID) -> Option<&Board> {
        self.entries.get(&id)
    }

    fn _parent(&self, id: BoardID) -> Option<BoardID> {
        self.parent.get(&id).cloned()
    }

    fn _children(&self, id: BoardID) -> Option<&[BoardID]> {
        self.children.get(&id).map(Vec::as_slice)
    }
}

#[derive(Default)]
pub struct SolveStats {
    tree: BoardTree,
    pub num_checked_boards: usize,
    pub num_skiped_single: usize,
}

impl SolveStats {
    pub fn insert_board(&mut self, parent: Option<BoardID>, board: Board) -> BoardID {
        self.num_checked_boards += 1;
        self.tree.insert(board, parent)
    }
}

pub trait Solvable {
    fn solve(&self, stats: &mut SolveStats, board: &Board, pieces: &[Vec<Piece>]) -> SolveResult;
}

pub mod dumb_solver;

pub fn solve<S: Solvable>(
    solver: S,
    board: &Board,
    pieces: &[Vec<Piece>],
) -> (SolveResult, SolveStats) {
    let mut stats = SolveStats::default();
    (solver.solve(&mut stats, board, pieces), stats)
}
