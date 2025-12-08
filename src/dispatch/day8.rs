use std::ops::{Add, Sub};

use anyhow::anyhow;

/// Connect the *1000* closest-together pairs of boxes to form some number of
/// circuits. Find the sizes of the *three* largest circuits, and multiply them.
pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    part1_real(input, 1000)
}

pub fn part1_real(input: &str, connect: usize) -> anyhow::Result<String> {
    Err(anyhow!("not implemented"))
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("not implemented"))
}

const _EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

// hey notably: this only takes 10 shortest, and the real thing takes 1000. So
// we need diverging impls.
#[test]
fn part1_test() {
    assert_eq!(
        part1_real(_EXAMPLE, 10).expect("should ok"),
        "40".to_string()
    );
}

#[test]
fn part2_test() {
    assert_eq!(part2(_EXAMPLE).expect("should ok"), "LOL".to_string());
}

#[derive(Clone, Copy, Debug, Default)]
struct Pt {
    x: i64,
    y: i64,
    z: i64,
}

impl Pt {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
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
    fn distance_squared(&self, other: &Self) -> i64 {
        let diff_sqd = (*self - *other).square_components();
        diff_sqd.x + diff_sqd.y + diff_sqd.z
    }
}

impl Sub for Pt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Pt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
