use std::num::ParseIntError;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Default)]
pub struct Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vec3 {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn from_str(line: &str) -> Result<Vec3, ParseIntError> {
        let mut stuff = line.split(',');
        let x = stuff.next().unwrap_or_default().parse::<i64>()?;
        let y = stuff.next().unwrap_or_default().parse::<i64>()?;
        let z = stuff.next().unwrap_or_default().parse::<i64>()?;
        Ok(Vec3::new(x, y, z))
    }

    fn square_components(&self) -> Self {
        Self {
            x: self.x * self.x,
            y: self.y * self.y,
            z: self.z * self.z,
        }
    }

    /// So, the Z-axis is perpendicular to any segment on the x/y plane. Thus if
    /// you start at the end of a segment and go in + or - Z for some distance,
    /// you form the legs of a right triangle. The original segment was already
    /// the hypotenuse of an earlier right triangle. So you end up with x^2 +
    /// y^2 = c^2 (where c is the x/y segment), then c^2 + z^2 = d^2 (where d is
    /// the distance we originally wanted). Notably we never have to square-root
    /// c, we can just turn straight around and feed it back.
    ///
    /// And as long as we're only *comparing* distances and not actually
    /// measuring them, we never have to unsquare d either, because n^2 > m^2
    /// whenever n > m.
    pub fn distance_squared(&self, other: &Self) -> i64 {
        let diff_sqd = (*self - *other).square_components();
        diff_sqd.x + diff_sqd.y + diff_sqd.z
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Default)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Vec2 {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn from_str(line: &str) -> Result<Self, ParseIntError> {
        let mut stuff = line.split(',');
        let x = stuff.next().unwrap_or_default().parse::<i64>()?;
        let y = stuff.next().unwrap_or_default().parse::<i64>()?;
        Ok(Self::new(x, y))
    }

    fn square_components(&self) -> Self {
        Self {
            x: self.x * self.x,
            y: self.y * self.y,
        }
    }

    /// So, the Z-axis is perpendicular to any segment on the x/y plane. Thus if
    /// you start at the end of a segment and go in + or - Z for some distance,
    /// you form the legs of a right triangle. The original segment was already
    /// the hypotenuse of an earlier right triangle. So you end up with x^2 +
    /// y^2 = c^2 (where c is the x/y segment), then c^2 + z^2 = d^2 (where d is
    /// the distance we originally wanted). Notably we never have to square-root
    /// c, we can just turn straight around and feed it back.
    ///
    /// And as long as we're only *comparing* distances and not actually
    /// measuring them, we never have to unsquare d either, because n^2 > m^2
    /// whenever n > m.
    pub fn distance_squared(&self, other: &Self) -> i64 {
        let diff_sqd = (*self - *other).square_components();
        diff_sqd.x + diff_sqd.y
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
