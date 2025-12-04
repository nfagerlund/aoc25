use anyhow::anyhow;

/// Find all *occupied* cells where fewer than four of the eight surrounding
/// cells are occupied.
pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let grid = Grid::build(input)?;
    let all_roll_indices = grid.iter_occupied_indices();
    let all_roll_neighbor_counts = all_roll_indices.map(|i| {
        let coords = grid.coords(i);
        grid.count_occupied_neighbors(coords)
    });
    let accessible_rolls_count = all_roll_neighbor_counts.filter(|c| *c < 4).count();
    Ok(format!("{accessible_rolls_count}"))
}

/// Iteratively count how many rolls of paper can be removed, according to the
/// accessibility rules in part 1. Each iteration exposes more rolls.
pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let mut grid = Grid::build(input)?;
    let mut count = 0;

    loop {
        let iteration_count = grid.evict_and_count();
        if iteration_count == 0 {
            break;
        }
        count += iteration_count;
    }

    Ok(format!("{count}"))
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
    assert_eq!(part2(_EXAMPLE).expect("should ok"), "43".to_string());
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
fn traverse(coords: Coords, dir: Dir) -> Option<Coords> {
    let (x, y) = coords;
    let n = match dir {
        Dir::E => (x + 1, y),
        Dir::NE => (x + 1, y.checked_sub(1)?),
        Dir::N => (x, y.checked_sub(1)?),
        Dir::NW => (x.checked_sub(1)?, y.checked_sub(1)?),
        Dir::W => (x.checked_sub(1)?, y),
        Dir::SW => (x.checked_sub(1)?, y + 1),
        Dir::S => (x, y + 1),
        Dir::SE => (x + 1, y + 1),
    };
    Some(n)
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
    fn _get(&self, coords: Coords) -> Option<&T> {
        let index = self.index(coords)?;
        self.storage.get(index)
    }

    fn _get_by_index(&self, index: usize) -> Option<&T> {
        self.storage.get(index)
    }

    fn get_neighbor(&self, coords: Coords, dir: Dir) -> Option<&T> {
        let neighbor = traverse(coords, dir)?;
        let index = self.index(neighbor)?;
        self.storage.get(index)
    }
}

impl Grid<bool> {
    fn build(ascii: &str) -> anyhow::Result<Self> {
        let width = width_of_ascii_grid(ascii);
        let stuff = load_ascii_grid_to_vec_of_bools(ascii);
        Grid::try_new(width, stuff)
    }

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

    fn iter_occupied_indices(&self) -> impl Iterator<Item = usize> {
        self.storage.iter().enumerate().filter_map(
            |(index, value)| {
                if *value { Some(index) } else { None }
            },
        )
    }

    // I realized we can just evict as-we-go, because there's no requirement for
    // instantaneous/simultaneous processing of a given iteration; we just need
    // to eventually converge, so it's fine if we exploit some slots that opened
    // up due to actions we just took. Good, even.
    fn evict_and_count(&mut self) -> usize {
        let mut count = 0;
        for i in 0..self.storage.len() {
            if let Some(occupied) = self.storage.get(i)
                && *occupied
            {
                let coords = self.coords(i);
                if self.count_occupied_neighbors(coords) < 4 {
                    count += 1;
                    self.storage[i] = false;
                }
            }
        }
        count
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
