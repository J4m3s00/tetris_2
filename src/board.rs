use std::fmt::Display;

use crate::{piece::Piece, Position};

/// The board has 8x8 fields.
/// Every field is a u8 to store an id for the current piece on the board
#[derive(Debug, Clone, Copy)]
pub struct Board([u8; 64]);

impl Default for Board {
    fn default() -> Self {
        Self([0; 64])
    }
}

impl Board {
    pub fn get_value(&self, pos: Position) -> u8 {
        let index = pos.y() * 8 + pos.x();
        self.0.get(index as usize).cloned().unwrap_or(0u8)
    }

    pub fn set_value(&mut self, pos: Position, new_value: u8) {
        let index = pos.y() * 8 + pos.x();
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
                if self.get_value(Position::new(x, y)) == 0 {
                    "X"
                } else {
                    " "
                }
            )?;
        }
        writeln!(f, "")
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
