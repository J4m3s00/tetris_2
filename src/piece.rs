use std::fmt::Display;

use crate::Position;

// +---+---+---+---+       +---+---+---+---+
// | X | X | X |   |       | X | X | X |   |
// +---+---+---+---+       +---+---+---+---+
// | X |   |   |   |       | X | X | X |   |
// +---+---+---+---+       +---+---+---+---+
//
// +---+---+---+---+       +---+---+---+---+
// | X | X | X | X |       | X | X |   |   |
// +---+---+---+---+       +---+---+---+---+
// | X |   |   |   |       |   | X | X |   |
// +---+---+---+---+       +---+---+---+---+
//
// +---+---+---+---+       +---+---+---+---+
// | X | X |   |   |       | X | X | X | X |
// +---+---+---+---+       +---+---+---+---+
// |   | X | X |   |       |   | X |   |   |
// +---+---+---+---+       +---+---+---+---+
//
// +---+---+---+---+       +---+---+---+---+
// |   | X |   |   |       | X | X |   |   |
// +---+---+---+---+       +---+---+---+---+
// | X | X | X |   |       | X | X |   |   |
// +---+---+---+---+       +---+---+---+---+
//
// +---+---+---+---+       +---+---+---+---+
// | X | X | X |   |       | X | X |   |   |
// +---+---+---+---+       +---+---+---+---+
// | X |   | X |   |       |   | X | X | X |
// +---+---+---+---+       +---+---+---+---+
//
// +---+---+---+---+       +---+---+---+---+
// | X |   | X |   |       | X |   |   |   |
// +---+---+---+---+       +---+---+---+---+
// | X | X | X |   |       | X |   |   |   |
// +---+---+---+---+       +---+---+---+---+
// | X |   | X |   |       | X | X | X |   |
// +---+---+---+---+       +---+---+---+---+
//
// +---+---+---+---+       +---+---+---+---+
// |   | X |   |   |       |   |   | X |   |
// +---+---+---+---+       +---+---+---+---+
// | X | X |   |   |       | X | X | X |   |
// +---+---+---+---+       +---+---+---+---+
// |   | X | X |   |       | X |   |   |   |
// +---+---+---+---+       +---+---+---+---+
// Rotations:
//     Normal                 CW                 CCW                  180
// +---+---+---+---+   +---+---+---+---+   +---+---+---+---+   +---+---+---+---+
// |   | X |   |   |   |   | X |   |   |   |   |   | X |   |   | X | X |   |   |
// +---+---+---+---+   +---+---+---+---+   +---+---+---+---+   +---+---+---+---+
// | X | X |   |   |   | X | X | X |   |   | X | X | X |   |   |   | X | X |   |
// +---+---+---+---+   +---+---+---+---+   +---+---+---+---+   +---+---+---+---+
// |   | X | X |   |   | X |   |   |   |   |   | X |   |   |   |   | X |   |   |
// +---+---+---+---+   +---+---+---+---+   +---+---+---+---+   +---+---+---+---+
// [(1, 0), (0, 1),    [(2, 1), (1, 0),    [(0, 1), (1, 2),    [(1, 2), (2, 1),
//  (1, 1), (1, 2),     (1, 1), (0, 1),     (1, 1), (2, 1),     (1, 1), (1, 0),
//  (2, 2)]             (0, 2)]             (2, 0)]             (0, 0)]

/// A Piece that can be placed to the [Board](crate::Board)
///
/// The id is the value that will be written in the board to differentiate between different pieces
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    id: u8,
    points: Vec<Position>,
}

impl Piece {
    /// Creates new piece
    pub fn new(id: u8, points: Vec<Position>) -> Self {
        assert_ne!(id, 0);
        Self { id, points }
    }

    /// Returns id of the piece
    pub fn id(&self) -> u8 {
        self.id
    }

    /// Gets slice of all the points of the piece
    pub fn points(&self) -> &[Position] {
        &self.points
    }

    /// Rotate the piece clock wise
    pub fn rotate_cw(&self) -> Self {
        self.transform_points(|point| {
            let x = point.x() as i8;
            let y = point.y() as i8;

            let x_1 = -y;
            let y_1 = x;

            (x_1, y_1)
        })
    }

    /// Rotate the piece counter clock wise
    pub fn rotate_ccw(&self) -> Self {
        self.transform_points(|point| {
            let x = point.x() as i8;
            let y = point.y() as i8;

            let x_1 = y;
            let y_1 = -x;

            (x_1, y_1)
        })
    }

    /// Rotate the piece 180 degrees
    pub fn rotate_180(&self) -> Self {
        self.transform_points(|point| {
            let x = point.x() as i8;
            let y = point.y() as i8;

            let x_1 = -x;
            let y_1 = -y;

            (x_1, y_1)
        })
    }

    /// Flip the piece on the x axis
    pub fn flip_x(&self) -> Self {
        self.transform_points(|point| {
            let x = point.x() as i8;
            let y = point.y() as i8;

            (-x, y)
        })
    }

    /// Flip the piece on the y axis
    pub fn flip_y(&self) -> Self {
        self.transform_points(|point| {
            let x = point.x() as i8;
            let y = point.y() as i8;

            (x, -y)
        })
    }

    /// Get all possible transforms.
    /// This will filter out duplicates automatically
    ///
    /// - self
    /// - rotated CW
    /// - rotated CCW
    /// - rotated 180
    /// - and all above flipped on both axis
    pub fn get_all_transforms(&self) -> Vec<Self> {
        let all = vec![
            self.clone(),
            self.rotate_cw(),
            self.rotate_180(),
            self.rotate_ccw(),
            self.flip_x(),
            self.flip_x().rotate_cw(),
            self.flip_x().rotate_180(),
            self.flip_x().rotate_ccw(),
            self.flip_y(),
            self.flip_y().rotate_cw(),
            self.flip_y().rotate_180(),
            self.flip_y().rotate_ccw(),
        ];
        let mut res: Vec<Piece> = Vec::new();
        for item in all {
            if !res.iter().any(|f| f.points == item.points) {
                res.push(item);
            }
        }
        res
    }

    /// Transform the points and place them back in the unsigned space.
    /// This will pack them as close as possible to the (0, 0)
    fn transform_points<F>(&self, f: F) -> Self
    where
        F: FnMut(&Position) -> (i8, i8),
    {
        let flipped_signed_points = self.points.iter().map(f).collect::<Vec<_>>();
        // Find the min value, so we can offset all of the values by that value, to get it in the unsigned range again
        let (offset_x, offset_y) = flipped_signed_points.iter().fold((0, 0), |a, b| {
            let x_offset = if b.0 < 0 { a.0.max(b.0.abs()) } else { a.0 };
            let y_offset = if b.1 < 0 { a.1.max(b.1.abs()) } else { a.1 };
            (x_offset, y_offset)
        });

        let mut points = flipped_signed_points
            .into_iter()
            .map(|(x, y)| Position::new((x + offset_x) as u8, (y + offset_y) as u8))
            .collect::<Vec<_>>();
        points.sort();

        Self {
            id: self.id,
            points,
        }
    }

    /// Returns the size of the piece
    fn bounds(&self) -> (u8, u8) {
        self.points.iter().fold((0, 0), |(w, h), point| {
            (w.max(point.x() + 1), h.max(point.y() + 1))
        })
    }

    /// Check if a point on a specific location exists
    fn check_point(&self, point: Position) -> bool {
        self.points.iter().any(|p| p == &point)
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bounds = self.bounds();
        for _ in 0..bounds.0 {
            write!(f, "+---")?;
        }
        writeln!(f, "+")?;

        for y in 0..bounds.1 {
            for x in 0..bounds.0 {
                write!(
                    f,
                    "| {} ",
                    if self.check_point(Position::new(x, y)) {
                        "X"
                    } else {
                        " "
                    }
                )?;
            }
            writeln!(f, "|")?;
            for _ in 0..bounds.0 {
                write!(f, "+---")?;
            }
            writeln!(f, "+")?;
        }

        Ok(())
    }
}
