use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hasher},
};

use crate::board::Board;

pub type BoardID = u64;

#[derive(Default)]
pub struct BoardTree {
    children: HashMap<BoardID, Vec<BoardID>>,
    parent: HashMap<BoardID, BoardID>,
    entries: HashMap<BoardID, Board>,
}

impl BoardTree {
    pub fn insert(&mut self, board: Board, parent: Option<BoardID>) -> BoardID {
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
