#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Orientation {
    // 0 degree Orientation
    Origin,
    // clockwise Orientation 90 degrees
    Right,
    // 180 degree Orientation
    Around,
    // counter clockwise Orientation 90 degrees
    Left,
}

impl Orientation {
    pub const COUNT: usize = 4;

    fn from(orientation: u64) -> Self {
        match orientation % Orientation::COUNT as u64 {
            0 => Self::Origin,
            1 => Self::Right,
            2 => Self::Around,
            3 => Self::Left,
            _ => unreachable!(),
        }
    }

    pub fn rotated_clockwise(self) -> Self {
        Self::from(self as u64 + 1)
    }

    pub fn rotated_counter_clockwise(self) -> Self {
        Self::from(self as u64 + Orientation::COUNT as u64 - 1)
    }

    pub fn rotated(self, direction: Direction) -> Self {
        match direction {
            Direction::Clockwise => self.rotated_clockwise(),
            Direction::CounterClockwise => self.rotated_counter_clockwise(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl Direction {
    pub const COUNT: usize = 2;
}
