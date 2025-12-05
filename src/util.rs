use anyhow::anyhow;
use std::ops::RangeInclusive;

/// Turns a string like "5-8" into a RangeInclusive.
pub fn parse_range(txt: &str) -> Result<RangeInclusive<u64>, anyhow::Error> {
    let (first, second) = txt.split_once('-').ok_or(anyhow!("not hyphenated pair"))?;
    let (start, end) = (first.trim().parse::<u64>()?, second.trim().parse::<u64>()?);

    Ok(start..=end)
}
