use std::fmt::Display;

use crate::{piece::Piece, Position};

/// The board has 8x8 fields.
/// Every field is a u8 to store an id for the current piece on the board
#[derive(Debug, Clone, Copy, Hash)]
pub struct Board([u8; 64]);

impl Default for Board {
    fn default() -> Self {
        Self([0; 64])
    }
}

impl Board {
    pub fn as_slice(&self) -> &[u8; 64] {
        &self.0
    }

    pub fn is_solved(&self) -> bool {
        self.0.iter().all(|v| *v != 0)
    }

    pub fn get_value(&self, position: Position) -> u8 {
        let index = position.y() * 8 + position.x();
        self.0.get(index as usize).cloned().unwrap_or(0u8)
    }

    pub fn set_value(&mut self, position: Position, new_value: u8) {
        let index = position.y() * 8 + position.x();
        if let Some(val) = self.0.get_mut(index as usize) {
            *val = new_value;
        }
    }

    pub fn place_piece(&mut self, top_left: Position, piece: &Piece) {
        if self.can_place_piece(top_left, piece) {
            for point in piece.points() {
                // We can unwrap, because we check that in the can_place_piece function
                self.set_value(top_left.try_add(point).unwrap(), piece.id());
            }
        } else {
            println!("Cant place piece!");
        }
    }

    pub fn can_place_piece(&self, top_left: Position, piece: &Piece) -> bool {
        piece
            .points()
            .iter()
            .all(|point| match top_left.try_add(point) {
                Some(p) => self.get_value(p) == 0,
                None => false,
            })
    }

    fn fmt_line(&self, f: &mut std::fmt::Formatter<'_>, y: u8) -> std::fmt::Result {
        write!(f, "|")?;
        for x in 0..8 {
            write!(
                f,
                " {} |",
                match self.get_value(Position::new(x, y)) {
                    0 => ' ',
                    v => {
                        (b'A' + (v - 1)) as char
                    }
                }
            )?;
        }
        writeln!(f)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+---+---+---+---+---+---+---+---+")?;
        self.fmt_line(f, 0)?;
        writeln!(f, "+---+---+---+---+---+---+---+---+")?;
        self.fmt_line(f, 1)?;
        writeln!(f, "+---+---+---+---+---+---+---+---+")?;
        self.fmt_line(f, 2)?;
        writeln!(f, "+---+---+---+---+---+---+---+---+")?;
        self.fmt_line(f, 3)?;
        writeln!(f, "+---+---+---+---+---+---+---+---+")?;
        self.fmt_line(f, 4)?;
        writeln!(f, "+---+---+---+---+---+---+---+---+")?;
        self.fmt_line(f, 5)?;
        writeln!(f, "+---+---+---+---+---+---+---+---+")?;
        self.fmt_line(f, 6)?;
        writeln!(f, "+---+---+---+---+---+---+---+---+")?;
        self.fmt_line(f, 7)?;
        writeln!(f, "+---+---+---+---+---+---+---+---+")
    }
}

#[cfg(test)]
mod tests {
    use crate::{piece::Piece, position::Position};

    use super::Board;

    #[test]
    fn get_set_value() {
        let mut board = Board::default();
        board.set_value(Position::new(4, 4), 32);
        assert_eq!(
            board.get_value(Position::new(4, 4)),
            32,
            "Found the value we just set"
        )
    }

    #[test]
    fn is_solved() {
        let mut board = Board::default();
        for y in 0..8 {
            for x in 0..8 {
                let position = Position::new(x, y);
                board.set_value(position, 1);
            }
        }
        assert!(board.is_solved(), "Board not solved");
    }

    #[test]
    fn place_piece() {
        // +---+---+---+---+
        // | X | X | X |   |
        // +---+---+---+---+
        // | X |   |   | X |
        // +---+---+---+---+
        // | X | X | X |   |
        // +---+---+---+---+

        let piece = Piece::new(
            1,
            vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(2, 0),
                Position::new(0, 1),
                Position::new(3, 1),
                Position::new(0, 2),
                Position::new(1, 2),
                Position::new(2, 2),
            ],
        );

        {
            test_single_piece_place(Board::default(), &piece, Position::new(0, 0));
        }
        {
            test_single_piece_place(Board::default(), &piece, Position::new(3, 3));
        }
        {
            let mut board = Board::default();
            board.set_value(Position::new(3, 0), 2);
            board.set_value(Position::new(1, 1), 2);
            board.set_value(Position::new(2, 1), 2);
            board.set_value(Position::new(3, 2), 2);
            test_single_piece_place(board, &piece, Position::new(0, 0));
        }
        {
            let mut board = Board::default();
            board.set_value(Position::new(0, 1), 2);
            assert!(!board.can_place_piece(Position::new(0, 0), &piece));
        }
    }

    fn test_single_piece_place(mut board: Board, piece: &Piece, position: Position) {
        board.place_piece(position, &piece);
        for p in piece.points() {
            assert_eq!(
                board.get_value(
                    position
                        .try_add(p)
                        .expect("Failed to get position of piece point")
                ),
                1
            );
        }
    }
}
