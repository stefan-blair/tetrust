
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    pub fn diag(r: i32) -> Self {
        Self(r, r)
    }

    pub fn unit_y(y: i32) -> Self {
        Self(0, y)
    }

    pub fn unit_x(x: i32) -> Self {
        Self(x, 0)
    }

    pub fn x(self) -> i32 {
        self.0
    }

    pub fn y(self) -> i32 {
        self.1
    }

    pub fn min(self) -> i32 {
        std::cmp::min(self.0, self.1)
    }

    pub fn max(self) -> i32 {
        std::cmp::max(self.0, self.1)
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl std::ops::Div for Point {
    type Output = Point;

    fn div(self, other: Self) -> Self {
        Point(self.0 / other.0, self.1 / other.1)
    }
}

impl std::ops::Mul for Point {
    type Output = Point;

    fn mul(self, other: Self) -> Self {
        Point(self.0 * other.0, self.1 * other.1)
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::new(0, 0)
    }
}

/**
 * Points supporting float values, used to describe tetrimino blocks relative
 * to their pivot point.
 */
#[derive(Clone, Copy)]
pub struct PartialPoint(pub f32, pub f32);

impl PartialPoint {
    pub const fn to_point(self) -> Point {
        let mut rounded_x = self.0 as i32;
        if rounded_x as f32 > self.0 {
            rounded_x -= 1;
        }

        let mut rounded_y = self.1 as i32;
        if rounded_y as f32 > self.1 {
            rounded_y -= 1;
        }
        Point(rounded_x, rounded_y)
    }

    pub const fn rotate_clockwise(self) -> Self {
        PartialPoint(self.1, -self.0)
    }
}
