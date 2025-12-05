use crate::util::parse_range;
use anyhow::anyhow;
use std::{cmp::Ordering, ops::RangeInclusive};

// Count how many ingredient IDs are in at least one fresh range.
pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let (ranges, ids) = parse_inputs(input)?;
    let count = ids
        .iter()
        .filter(|&n| ranges.iter().any(|r| r.contains(n)))
        .count();

    Ok(format!("{count}"))
}

// Count how many possible fresh ingredient IDs are described by the ranges,
// ignoring actual provided IDs and deduplicating overlapping ranges.
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
    assert_eq!(part2(_EXAMPLE).expect("should ok"), "14".to_string());
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

/// what it sez
fn cmp_ranges(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> Ordering {
    match a.start().cmp(b.start()) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => a.end().cmp(b.end()),
        Ordering::Greater => Ordering::Greater,
    }
}

/// only cares about forward overlap, i.e. b is equal to or greater than a. Since we're sorting.
fn overlaps(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    a.start() <= b.start() && a.end() >= b.start()
}

/// Combines two ranges into mega-range.
/// be sure to check with overlaps() first, because we ain't checkin' shit.
fn merge(a: RangeInclusive<u64>, b: RangeInclusive<u64>) -> RangeInclusive<u64> {
    let start = *a.start().min(b.start());
    let end = *a.end().max(b.end());
    start..=end
}

#[test]
fn overlaps_test() {
    // overlaps but doesn't go further
    assert!(overlaps(&(0..=9), &(0..=8)));
    // fully subsumed
    assert!(overlaps(&(0..=9), &(1..=8)));
    // goes further
    assert!(overlaps(&(0..=9), &(5..=12)));
    // exact handoff
    assert!(overlaps(&(0..=9), &(9..=15)));

    // disjoint
    assert!(!overlaps(&(0..=9), &(10..=15)));
}

#[test]
fn merge_test() {
    // [a  ]
    //    [b  ]
    assert_eq!(merge(1..=5, 4..=9), 1..=9);
    // [a  ]
    // [b    ]
    assert_eq!(merge(2..=5, 2..=9), 2..=9);
    // [a     ]
    //    [b  ]
    assert_eq!(merge(1..=5, 4..=5), 1..=5);
    //    [a  ]
    // [b   ]
    assert_eq!(merge(4..=9, 1..=5), 1..=9);
    // [a  ]
    //      [b  ]
    assert_eq!(merge(1..=5, 5..=9), 1..=9);
}

