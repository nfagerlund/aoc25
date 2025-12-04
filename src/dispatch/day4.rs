use anyhow::anyhow;

/// Find all *occupied* cells where fewer than four of the eight surrounding
/// cells are occupied.
pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("not implemented"))
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("not implemented"))
}

const _EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

#[test]
fn part1_test() {
    assert_eq!(part1(_EXAMPLE).expect("should ok"), "13".to_string());
}

#[test]
fn part2_test() {
    assert_eq!(part1(_EXAMPLE).expect("should ok"), "LOL".to_string());
}

// Can I implement a grid without making the underlying array two-dimensional?

/// Zero-indexed coordinates, positive Y is down (like raster graphics).
struct Grid<T> {
    storage: Vec<T>,
    width: usize,
}

type Coords = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    // unit circle order for autocomplete
    E,
    NE,
    N,
    NW,
    W,
    SW,
    S,
    SE,
}

impl Dir {
    const ALL: &[Self] = &[
        Self::E,
        Self::NE,
        Self::N,
        Self::NW,
        Self::W,
        Self::SW,
        Self::S,
        Self::SE,
    ];
}

/// positive Y is south. Ignoring grid size.
fn traverse(coords: Coords, dir: Dir) -> Coords {
    let (x, y) = coords;
    match dir {
        Dir::E => (x + 1, y),
        Dir::NE => (x + 1, y - 1),
        Dir::N => (x, y - 1),
        Dir::NW => (x - 1, y - 1),
        Dir::W => (x - 1, y),
        Dir::SW => (x - 1, y + 1),
        Dir::S => (x, y + 1),
        Dir::SE => (x + 1, y + 1),
    }
}

impl<T> Grid<T> {
    fn try_new(width: usize, stuff: Vec<T>) -> anyhow::Result<Self> {
        if !stuff.len().is_multiple_of(width) {
            Err(anyhow!(
                "storage length {} is not a multiple of width {}",
                stuff.len(),
                width
            ))
        } else {
            Ok(Self {
                storage: stuff,
                width,
            })
        }
    }

    /// Returns none if x exceeds width.
    fn index(&self, coords: Coords) -> Option<usize> {
        let (x, y) = coords;
        // zero-indexed, so max x in a 5-width is 4
        if x >= self.width {
            None
        } else {
            Some(x + (y * self.width))
        }
    }

    fn coords(&self, index: usize) -> Coords {
        let y = index / self.width;
        let x = index % self.width;
        (x, y)
    }

    /// Get the value at a grid cell
    fn get(&self, coords: Coords) -> Option<&T> {
        let index = self.index(coords)?;
        self.storage.get(index)
    }

    fn get_neighbor(&self, coords: Coords, dir: Dir) -> Option<&T> {
        let neighbor = traverse(coords, dir);
        let index = self.index(neighbor)?;
        self.storage.get(index)
    }
}

impl Grid<bool> {
    fn count_occupied_neighbors(&self, coords: Coords) -> usize {
        Dir::ALL
            .iter()
            .filter_map(|&dir| {
                let v = self.get_neighbor(coords, dir)?;
                match v {
                    true => Some(true),
                    false => None,
                }
            })
            .count()
    }
}

fn load_ascii_grid_to_vec_of_bools(input: &str) -> Vec<bool> {
    let stuff = input.bytes().filter_map(|b| {
        match b {
            b'\n' => None,
            b'@' => Some(true),
            b'.' => Some(false),
            _ => None, // shouldn't happen
        }
    });
    Vec::from_iter(stuff)
}

fn width_of_ascii_grid(input: &str) -> usize {
    input
        .lines()
        .next()
        .expect("empty string, just gonna panic early")
        .len()
}

#[test]
fn width_test() {
    assert_eq!(width_of_ascii_grid(_EXAMPLE), 10);
    // assert_eq!(width_of_ascii_grid(""), 0); // nope
    assert_eq!(width_of_ascii_grid("\n"), 0);
}

#[test]
fn byte_char_test() {
    let s = "
hey";
    assert_eq!(s.bytes().next(), Some(b'\n'));
}
