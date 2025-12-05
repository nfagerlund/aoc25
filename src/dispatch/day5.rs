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
    let (ranges, _) = parse_inputs(input)?;
    let initial_len = ranges.len();
    let merged_ranges = compact_ranges(ranges);
    let merged_len = merged_ranges.len();
    println!("Reduced from {initial_len} to {merged_len}");
    let count = merged_ranges
        .iter()
        .map(r_len)
        .reduce(|a, b| a + b)
        .ok_or(anyhow!("compacted list of ranges shouldn't be empty??"))?;

    Ok(format!("{count}"))
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
    // this is subtle, see the tests.
    *a.start() <= *b.start() && b.start().saturating_sub(*a.end()) <= 1
}

/// Combines two ranges into mega-range.
/// be sure to check with overlaps() first, because we ain't checkin' shit.
fn merge(a: RangeInclusive<u64>, b: RangeInclusive<u64>) -> RangeInclusive<u64> {
    let start = *a.start().min(b.start());
    let end = *a.end().max(b.end());
    start..=end
}

/// oh huh weird that there's no impl ExactSizeIterator for RangeInclusive<u64>.
/// I guess usize is u32 on 32-bit platforms.....
fn r_len(r: &RangeInclusive<u64>) -> u64 {
    r.end() + 1 - r.start()
}

#[test]
fn r_len_test() {
    assert_eq!(r_len(&(1..=4)), 4);
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
    // adjacent counts, actually!! bc these are integers, not fractions.
    // there is no int between 9 and 10 not covered by the range.
    assert!(overlaps(&(0..=9), &(10..=15)));

    // disjoint, 10 is in the gap
    assert!(!overlaps(&(0..=9), &(11..=15)));
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

/// Consume a collection of ranges and return a fully deduplicated collection of
/// ranges, with all overlapping ranges merged.
fn compact_ranges(mut input_ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    // overkill length by definition, but guarantees no third alloc
    let mut output_ranges = Vec::<RangeInclusive<u64>>::with_capacity(input_ranges.len());
    input_ranges.sort_by(cmp_ranges);

    let mut feed = input_ranges.into_iter();
    let mut current = feed
        .next()
        .expect("don't u play ding-dong-ditch with me young man");

    println!("  in: {:?}", &current);

    for next in feed {
        if overlaps(&current, &next) {
            println!("  in: {:?}", &next);
            current = merge(current, next);
        } else {
            // we hit a disjunction.
            println!("merged: {:?}", &current);
            println!("  in: {:?}", &next);
            output_ranges.push(current);
            current = next;
        }
    }
    // we're left with one dangling current range at the end.
    println!("merged: {:?}", &current);
    output_ranges.push(current);

    output_ranges
}
