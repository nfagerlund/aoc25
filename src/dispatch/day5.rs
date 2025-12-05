use crate::util::parse_range;
use anyhow::anyhow;
use std::ops::RangeInclusive;

// Count how many ingredient IDs are in at least one fresh range.
pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let (ranges, ids) = parse_inputs(input)?;
    let count = ids
        .iter()
        .filter(|&n| ranges.iter().any(|r| r.contains(n)))
        .count();

    Ok(format!("{count}"))
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("not implemented"))
}

// fresh ranges (inclusive), then available ids
const _EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

#[test]
fn part1_test() {
    assert_eq!(part1(_EXAMPLE).expect("should ok"), "3".to_string());
}

#[test]
fn part2_test() {
    assert_eq!(part2(_EXAMPLE).expect("should ok"), "LOL".to_string());
}

/// Turn the day5 input string into numeric types
fn parse_inputs(input: &str) -> anyhow::Result<(Vec<RangeInclusive<u64>>, Vec<u64>)> {
    let (ranges_str, ids_str) = input
        .split_once("\n\n")
        .ok_or(anyhow!("input not a double-newline-separated list pair"))?;
    // Since this is fallible, I want to use the ? operator to just propagate failure.
    // But that rules out iterator adaptors, alas, so it's a for loop.
    let mut ranges = Vec::<RangeInclusive<u64>>::new();
    for line in ranges_str.lines() {
        ranges.push(parse_range(line)?);
    }
    let mut ids = Vec::<u64>::new();
    for line in ids_str.lines() {
        ids.push(line.parse()?);
    }

    Ok((ranges, ids))
}
