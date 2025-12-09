use anyhow::anyhow;
use std::ops::RangeInclusive;

mod vec;
pub use vec::*;

/// Turns a string like "5-8" into a RangeInclusive.
pub fn parse_range(txt: &str) -> Result<RangeInclusive<u64>, anyhow::Error> {
    let (first, second) = txt.split_once('-').ok_or(anyhow!("not hyphenated pair"))?;
    let (start, end) = (first.trim().parse::<u64>()?, second.trim().parse::<u64>()?);

    Ok(start..=end)
}

/// A two-dimensional grid, implemented as a single-dimensional array that
/// translates coordinates <-> indices on the fly. Zero-indexed coordinates,
/// positive Y is down (like raster graphics).
pub struct Grid<T> {
    pub storage: Vec<T>,
    pub width: usize,
}

pub type Coords = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dir {
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
    pub const ALL: &[Self] = &[
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
pub fn traverse(coords: Coords, dir: Dir) -> Option<Coords> {
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
    pub fn try_new(width: usize, stuff: Vec<T>) -> anyhow::Result<Self> {
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
    pub fn index(&self, coords: Coords) -> Option<usize> {
        let (x, y) = coords;
        // zero-indexed, so max x in a 5-width is 4
        if x >= self.width {
            None
        } else {
            Some(x + (y * self.width))
        }
    }

    pub fn coords(&self, index: usize) -> Coords {
        let y = index / self.width;
        let x = index % self.width;
        (x, y)
    }

    /// Get the value at a grid cell
    pub fn get(&self, coords: Coords) -> Option<&T> {
        let index = self.index(coords)?;
        self.storage.get(index)
    }

    fn _get_by_index(&self, index: usize) -> Option<&T> {
        self.storage.get(index)
    }

    pub fn get_neighbor(&self, coords: Coords, dir: Dir) -> Option<&T> {
        let neighbor = traverse(coords, dir)?;
        self.get(neighbor)
    }
}

impl Grid<bool> {
    pub fn count_occupied_neighbors(&self, coords: Coords) -> usize {
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

    pub fn iter_occupied_indices(&self) -> impl Iterator<Item = usize> {
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
    pub fn evict_and_count(&mut self) -> usize {
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
