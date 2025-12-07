//! This module has all the nasty stuff that I need to fill via template. It
//! keeps it out of main. However, as a consequence, all the day modules need to
//! be children of it.

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub const DAYS: &[&str] = &["1", "2", "3", "4", "5", "6", "7"];

/// Maps puzzle identifiers to implementation functions. The main thing you
/// gotta add when starting a new day.
pub fn puzzle_fn(p: &str) -> fn(&str) -> Result<String, anyhow::Error> {
    match p {
        "1-1" => day1::part1,
        "1-2" => day1::part2,
        "2-1" => day2::part1,
        "2-2" => day2::part2,
        "3-1" => day3::part1,
        "3-2" => day3::part2,
        "4-1" => day4::part1,
        "4-2" => day4::part2,
        "5-1" => day5::part1,
        "5-2" => day5::part2,
        "6-1" => day6::part1,
        "6-2" => day6::part2,
        "7-1" => day7::part1,
        "7-2" => day7::part2,

        _ => panic!("That's not a valid puzzle yet"),
    }
}
