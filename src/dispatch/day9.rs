use std::num::ParseIntError;

use crate::util::Vec2;
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
