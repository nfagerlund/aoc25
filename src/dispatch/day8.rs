use std::{
    collections::HashSet,
    num::ParseIntError,
    ops::{Add, Sub},
};

use anyhow::anyhow;

use crate::util::{Coords, Grid};

/// Connect the *1000* closest-together pairs of boxes to form some number of
/// circuits. Find the sizes of the *three* largest circuits, and multiply them.
pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    part1_real(input, 1000)
}

pub fn part1_real(input: &str, connect: usize) -> anyhow::Result<String> {
    let points = load_points(input)?;
    let grid = grid_of_all_distances(&points)?;
    // uhhhhhh
    // transform and filter so we've got a deduplicated, sortable list of (len, coords).
    let mut connections: Vec<(i64, Coords)> = grid
        .storage
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(i, len)| {
            let coords = grid.coords(i);
            if coords.1 >= coords.0 {
                // then it's either a self-connect or a duplicate.
                None
            } else {
                Some((len, coords))
            }
        })
        .collect();
    // sort em
    connections.sort_by_key(|(len, _coords)| *len);
    // Build circuits... for a while.
    let mut circuits = Circuits::new();
    for (_len, (left, right)) in connections.into_iter().take(connect) {
        circuits.add_connection(points[left], points[right]);
    }
    // Sort em, multiply the three biggest lengths
    circuits.sort_descending();

    let product = circuits
        .stuff
        .iter()
        .take(3)
        .map(|set| set.len())
        .reduce(|acc, e| acc * e)
        .expect("Need at least 3 circuits");

    Ok(format!("{product}"))
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

fn pt_from_str(line: &str) -> Result<Pt, ParseIntError> {
    let mut stuff = line.split(',');
    let x = stuff.next().unwrap_or_default().parse::<i64>()?;
    let y = stuff.next().unwrap_or_default().parse::<i64>()?;
    let z = stuff.next().unwrap_or_default().parse::<i64>()?;
    Ok(Pt::new(x, y, z))
}

fn load_points(input: &str) -> Result<Vec<Pt>, ParseIntError> {
    // hahahahahahaha hell yeah
    input.lines().map(pt_from_str).collect()
}

/// The resulting grid contains (squared) distances. The scale of the X and Y
/// axes of the grid correspond to indices into the points slice, so we're
/// counting on you not re-sorting that. The grid has redundant info; we don't
/// want to double-count any connections or do any self-connectons. But the
/// ability to derive coords is too useful.
fn grid_of_all_distances(points: &[Pt]) -> anyhow::Result<Grid<i64>> {
    let mut storage = Vec::<i64>::with_capacity(points.len() * points.len());
    for y in points {
        for x in points {
            storage.push(x.distance_squared(y));
        }
    }
    Grid::try_new(points.len(), storage)
}

/// I'm gonna stop tracking the direct connections once you're in a circuit.
/// bugs bunny full communism dot jpg.
struct Circuits {
    stuff: Vec<HashSet<Pt>>,
}

impl Circuits {
    fn new() -> Self {
        Self { stuff: Vec::new() }
    }

    fn sort_descending(&mut self) {
        self.stuff.sort_by(|a, b| b.len().cmp(&a.len()));
    }

    /// returns the index of the circuit containing the specified point, if extant.
    fn idx_containing_point(&self, point: Pt) -> Option<usize> {
        self.stuff.iter().enumerate().find_map(
            |(i, set)| {
                if set.contains(&point) { Some(i) } else { None }
            },
        )
    }

    fn add_connection(&mut self, left: Pt, right: Pt) {
        let contains_left_idx = self.idx_containing_point(left);
        let contains_right_idx = self.idx_containing_point(right);
        match (contains_left_idx, contains_right_idx) {
            (None, None) => {
                let mut new = HashSet::new();
                new.insert(left);
                new.insert(right);
                self.stuff.push(new);
            }
            (None, Some(right_i)) => {
                self.stuff[right_i].insert(left);
            }
            (Some(left_i), None) => {
                self.stuff[left_i].insert(right);
            }
            (Some(left_i), Some(right_i)) => {
                // Merge em!!
                let (larger_i, smaller_i) = match left_i.cmp(&right_i) {
                    std::cmp::Ordering::Less => (right_i, left_i),
                    std::cmp::Ordering::Equal => {
                        // oops???? we've been here before
                        return;
                    }
                    std::cmp::Ordering::Greater => (left_i, right_i),
                };
                // Remove the further-out one, so the index of the other stays stable...
                let mut combined = self.stuff.remove(larger_i);
                let absorbee = self.stuff.get_mut(smaller_i).expect("impossible");
                combined.extend(absorbee.drain());
                // Swap the combined one into place
                self.stuff[smaller_i] = combined;
            }
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Default)]
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
