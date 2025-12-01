use std::fs;

// let's do a standard interface: dayN::partM(&str) -> String
// I figure most answers will probably be ints? but can't fully predict.
pub fn part1(input: &str) -> String {
    "lol".to_string()
}

#[test]
fn part1_test() {
    let input = "L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82
";
    assert_eq!(part1(input), "3".to_string());
}
