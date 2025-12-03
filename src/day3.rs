use anyhow::anyhow;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("unimplemented"))
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("unimplemented"))
}

const _EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

#[test]
fn part1_test() {
    assert_eq!(part1(_EXAMPLE).expect("should ok"), "357".to_string());
}

#[test]
fn part_2_test() {
    assert_eq!(
        part2(_EXAMPLE).expect("should ok"),
        "aseothuaestnhu".to_string()
    );
}
