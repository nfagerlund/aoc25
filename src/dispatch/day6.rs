use anyhow::anyhow;

/// Solve all the columnar addition-or-multiplication problems, and sum the answers.
pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("not implemented"))
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("not implemented"))
}

// Looks like they all stay positive. 🤗
const _EXAMPLE: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

#[test]
fn part1_test() {
    assert_eq!(part1(_EXAMPLE).expect("should ok"), "4277556".to_string());
}

#[test]
fn part2_test() {
    assert_eq!(part2(_EXAMPLE).expect("should ok"), "LOL".to_string());
}
