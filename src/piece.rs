use crate::Position;

/// +---+---+---+---+       +---+---+---+---+
/// | X | X | X |   |       | X | X | X |   |
/// +---+---+---+---+       +---+---+---+---+
/// | X |   |   |   |       | X | X | X |   |
/// +---+---+---+---+       +---+---+---+---+
///
/// +---+---+---+---+       +---+---+---+---+
/// | X | X | X | X |       | X | X |   |   |
/// +---+---+---+---+       +---+---+---+---+
/// | X |   |   |   |       |   | X | X |   |
/// +---+---+---+---+       +---+---+---+---+
///
/// +---+---+---+---+       +---+---+---+---+
/// | X | X |   |   |       | X | X | X | X |
/// +---+---+---+---+       +---+---+---+---+
/// |   | X | X |   |       |   | X |   |   |
/// +---+---+---+---+       +---+---+---+---+
///
/// +---+---+---+---+       +---+---+---+---+
/// |   | X |   |   |       | X | X |   |   |
/// +---+---+---+---+       +---+---+---+---+
/// | X | X | X |   |       | X | X |   |   |
/// +---+---+---+---+       +---+---+---+---+
///
/// +---+---+---+---+       +---+---+---+---+
/// | X | X | X |   |       | X | X |   |   |
/// +---+---+---+---+       +---+---+---+---+
/// | X |   | X |   |       |   | X | X | X |
/// +---+---+---+---+       +---+---+---+---+
///
/// +---+---+---+---+       +---+---+---+---+
/// | X |   | X |   |       | X |   |   |   |
/// +---+---+---+---+       +---+---+---+---+
/// | X | X | X |   |       | X |   |   |   |
/// +---+---+---+---+       +---+---+---+---+
/// | X |   | X |   |       | X | X | X |   |
/// +---+---+---+---+       +---+---+---+---+
///
/// +---+---+---+---+       +---+---+---+---+
/// |   | X |   |   |       |   |   | X |   |
/// +---+---+---+---+       +---+---+---+---+
/// | X | X |   |   |       | X | X | X |   |
/// +---+---+---+---+       +---+---+---+---+
/// |   | X | X |   |       | X |   |   |   |
/// +---+---+---+---+       +---+---+---+---+
/// Rotations:
///     Normal                 CW                 CCW                  180
/// +---+---+---+---+   +---+---+---+---+   +---+---+---+---+   +---+---+---+---+
/// |   | X |   |   |   |   | X |   |   |   |   |   | X |   |   | X | X |   |   |
/// +---+---+---+---+   +---+---+---+---+   +---+---+---+---+   +---+---+---+---+
/// | X | X |   |   |   | X | X | X |   |   | X | X | X |   |   |   | X | X |   |
/// +---+---+---+---+   +---+---+---+---+   +---+---+---+---+   +---+---+---+---+
/// |   | X | X |   |   | X |   |   |   |   |   | X |   |   |   |   | X |   |   |
/// +---+---+---+---+   +---+---+---+---+   +---+---+---+---+   +---+---+---+---+
/// [(1, 0), (0, 1),    [(2, 1), (1, 0),    [(0, 1), (1, 2),    [(1, 2), (2, 1),
///  (1, 1), (1, 2),     (1, 1), (0, 1),     (1, 1), (2, 1),     (1, 1), (1, 0),
///  (2, 2)]             (0, 2)]             (2, 0)]             (0, 0)]
#[derive(Clone)]
pub struct Piece {
    id: u8,
    points: Vec<Position>,
}

impl Piece {
    pub fn new(id: u8, points: Vec<Position>) -> Self {
        Self { id, points }
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn points(&self) -> &[Position] {
        &self.points
    }

    pub fn rotate_cw(&self) -> Self {
        self.transform_points(|point| {
            let x = point.x() as i8;
            let y = point.y() as i8;

            let x_1 = -y;
            let y_1 = x;

            (x_1, y_1)
        })
    }

    pub fn rotate_ccw(&self) -> Self {
        self.transform_points(|point| {
            let x = point.x() as i8;
            let y = point.y() as i8;

            let x_1 = y;
            let y_1 = -x;

            (x_1, y_1)
        })
    }

    pub fn rotate_180(&self) -> Self {
        self.transform_points(|point| {
            let x = point.x() as i8;
            let y = point.y() as i8;

            let x_1 = -x;
            let y_1 = -y;

            (x_1, y_1)
        })
    }

    pub fn flip_x(&self) -> Self {
        self.transform_points(|point| {
            let x = point.x() as i8;
            let y = point.y() as i8;

            (-x, y)
        })
    }

    pub fn flip_y(&self) -> Self {
        self.transform_points(|point| {
            let x = point.x() as i8;
            let y = point.y() as i8;

            (x, -y)
        })
    }

    pub fn get_all_transforms(&self) -> Vec<Self> {
        vec![
            self.clone(),
            self.rotate_cw(),
            self.rotate_180(),
            self.rotate_ccw(),
            self.flip_x(),
            self.flip_y(),
        ]
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

        Self {
            id: self.id,
            points: flipped_signed_points
                .into_iter()
                .map(|(x, y)| Position::new((x + offset_x) as u8, (y + offset_y) as u8))
                .collect(),
        }
    }
}
