use std::fs;

// let's do a standard interface: dayN::partM(&str) -> String
// I figure most answers will probably be ints? but can't fully predict.

/// Apply the supplied rotations to a wrapping 100-tick dial (labeled 0-99), and
/// count how many times the dial stops at 0.
pub fn part1(input: &str) -> String {
    // dial starts at 50
    let mut dial: i32 = 50;
    let mut zero_counter: u32 = 0;

    for rot in input.lines() {
        // oh right, gotta pipe err handling further back...
        let i = parse_rot_i32(rot).unwrap();
        println!("rotating {}...", i);
        dial += i;
        dial = wrap100(dial);
        println!("dial: {}", dial);
        if dial == 0 {
            zero_counter += 1;
        }
    }

    format!("{}", zero_counter)
}

#[derive(PartialEq, Debug)]
struct ParseError;

fn parse_rot_i32(rot: &str) -> Result<i32, ParseError> {
    let Some((dir, num)) = rot.trim().split_at_checked(1) else {
        return Err(ParseError);
    };
    // requiring uppercase
    let signum = match dir {
        "R" => 1,
        "L" => -1,
        _ => {
            return Err(ParseError);
        }
    };
    let parsed_num = num.parse::<i32>().map_err(|_| ParseError)?;

    Ok(signum * parsed_num)
}

fn wrap100(mut v: i32) -> i32 {
    while v < 0 {
        v += 100;
    }
    v % 100
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

#[test]
fn mod100() {
    assert_eq!(100_i32 % 100, 0);
    assert_eq!(-101_i32 % 100, -1);
    assert_eq!(-0_i32, 0);
    // assert_eq!(((50_i32 - 68) % 100).abs(), 82); // it's 18!
    assert_eq!(wrap100(50_i32 - 68), 82);
}

#[test]
fn parser() {
    assert_eq!(parse_rot_i32("L30"), Ok(-30));
    assert_eq!(parse_rot_i32("L101"), Ok(-101));
    assert_eq!(parse_rot_i32("R14"), Ok(14));
    assert!(parse_rot_i32("r14").is_err());
    assert!(parse_rot_i32("14").is_err());
}
