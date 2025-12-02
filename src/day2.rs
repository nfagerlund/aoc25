use anyhow::anyhow;
use std::ops::RangeInclusive;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("lol"))
}

const _EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

fn parse_range(txt: &str) -> Result<RangeInclusive<u32>, anyhow::Error> {
    let (first, second) = txt.split_once('-').ok_or(anyhow!("not hyphenated pair"))?;
    let (start, end) = (first.parse::<u32>()?, second.parse::<u32>()?);

    Ok(start..=end)
}

#[test]
fn parse_range_test() {
    assert_eq!(
        parse_range("38593856-38593862").expect("works"),
        38593856..=38593862
    );
    assert!(parse_range("hey-ho").is_err());
    assert!(parse_range("1234-").is_err());
    assert!(parse_range("-1234").is_err());
}
