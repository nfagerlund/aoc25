use std::num::ParseIntError;

use crate::util::{Coords, Grid, Vec2};
use anyhow::anyhow;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let stuff: Result<Vec<Vec2>, ParseIntError> = input.lines().map(Vec2::from_str).collect();
    let stuff = stuff?;
    let mut combinations = Vec::<i64>::with_capacity(stuff.len() * stuff.len() / 2);
    for i in 0..stuff.len() {
        if i + 1 >= stuff.len() {
            break;
        }
        for j in (i + 1)..stuff.len() {
            let h = stuff[i];
            let w = stuff[j];
            let diff = h - w;
            let area = (diff.x.abs() + 1) * (diff.y.abs() + 1);
            println!("{h} x {w}: area {area}");
            combinations.push(area);
        }
    }
    let max = combinations
        .iter()
        .copied()
        .max()
        .ok_or(anyhow!("empty combinations??"))?;

    Ok(format!("{max}"))
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("not implemented"))
}

const _EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

#[test]
fn part1_test() {
    assert_eq!(part1(_EXAMPLE).expect("should ok"), "50".to_string());
}

#[test]
fn part2_test() {
    assert_eq!(part2(_EXAMPLE).expect("should ok"), "24".to_string());
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Red,
    Green,
    Nah,
}

fn make_coords(pair: &str) -> anyhow::Result<Coords> {
    let (x, y) = pair.split_once(',').ok_or(anyhow!("not a pair"))?;
    Ok((x.parse()?, y.parse()?))
}

fn draw_line(grid: &mut Grid<Tile>, one: Coords, two: Coords) {
    let index_one = grid.index(one).expect("hey!!!");
    let index_two = grid.index(two).expect("hey!!!");
    grid.storage[index_one] = Tile::Red;
    grid.storage[index_two] = Tile::Red;
    let start_x = one.0.min(two.0);
    let start_y = one.1.min(two.1);
    let end_x = one.0.max(two.0);
    let end_y = one.1.max(two.1);
    for x in start_x..end_x {
        let index = grid.index((x, one.1)).expect("hey stop that");
        if grid.storage[index] == Tile::Nah {
            grid.storage[index] = Tile::Green;
        }
    }
    for y in start_y..end_y {
        let index = grid.index((one.0, y)).expect("hey stop that");
        if grid.storage[index] == Tile::Nah {
            grid.storage[index] = Tile::Green;
        }
    }
}
