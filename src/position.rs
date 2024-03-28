/// Position of a point in the board.
/// The coords will always be between 0 and 7
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    x: u8,
    y: u8,
}

impl Position {
    /// Creates new position.
    /// Will validate if the coords are 0 < [x, y] < 8
    pub fn new(x: u8, y: u8) -> Self {
        assert!(x < 8);
        assert!(y < 8);

        Self { x, y }
    }

    /// Returns the x coord
    pub fn x(&self) -> u8 {
        self.x
    }

    /// Returns the y coord
    pub fn y(&self) -> u8 {
        self.y
    }

    /// Trys adding a position to self
    ///
    /// Returns [None] if the position is not in the range 0..8
    pub fn try_add(&self, other: &Self) -> Option<Self> {
        let x = self.x + other.x;
        let y = self.y + other.y;
        if x < 8 && y < 8 {
            Some(Position { x, y })
        } else {
            None
        }
    }

    /// Will apply the offset to the positions
    ///
    /// This can be negativ and bigger than 8 afterwords
    pub fn offset(&self, offset: (i8, i8)) -> (i8, i8) {
        let x = self.x as i8;
        let y = self.y as i8;
        (x + offset.0, y + offset.1)
    }
}
