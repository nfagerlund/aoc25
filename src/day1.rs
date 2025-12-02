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

pub fn part2(input: &str) -> String {
    let mut dial: i32 = 50;
    let mut zero_counter: u32 = 0;

    for rot in input.lines() {
        let i = parse_rot_i32(rot).unwrap();
        println!("rotating {}...", i);
        let (d, z) = turn_wrap_and_count_zeros(dial, i);
        println!("new dial: {}, zero crossings: {}", d, z);
        dial = d;
        zero_counter += z;
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

/// This doesn't work the way we want!
fn _wrap100_and_count_zeroes(mut v: i32) -> (i32, u32) {
    let mut zeroes = 0_u32;
    while v < 0 {
        v += 100;
        zeroes += 1;
    }
    zeroes += v.div_euclid(100) as u32;
    let rem = v.rem_euclid(100);
    (rem, zeroes)
}

fn turn_wrap_and_count_zeros(start: i32, mut turn: i32) -> (i32, u32) {
    if !(0..=99).contains(&start) {
        panic!("Invalid dial position.");
    }
    if turn == 0 {
        // No movement to do
        return (start, 0);
    }

    let mut zeroes = 0_u32;
    let mut dial = start;

    // Ok we're moving. First, spend as much of the turn as necessary to move
    // the dial to 0 (if we have enough). If we make it, that's our first zero.
    if start != 0 {
        // Say we're at 16. If turn is negative, we want -16. If turn is positive, we want +84.
        let first_bite = if turn > 0 {
            (100 - start).min(turn)
        } else {
            // negative numbers making min/max funky. In both branches, we want the smallest abs.
            (-start).max(turn)
        };
        dial += first_bite;
        turn -= first_bite;
        dial %= 100;
        if dial == 0 {
            // we made it
            zeroes += 1;
        }
    }

    // Now we can spend the rest of the turn, first in zero-ing increments...
    while turn <= -100 {
        turn += 100;
        zeroes += 1;
    }
    while turn >= 100 {
        turn -= 100;
        zeroes += 1;
    }
    // ...then in the remainder.
    dial += turn;
    if dial < 0 {
        dial += 100;
    }

    (dial, zeroes)
}

#[test]
fn turn_wrap_count_test() {
    assert_eq!(turn_wrap_and_count_zeros(0, 100), (0, 1));
    assert_eq!(turn_wrap_and_count_zeros(1, 50), (51, 0));
    assert_eq!(turn_wrap_and_count_zeros(0, -50), (50, 0));
    assert_eq!(turn_wrap_and_count_zeros(0, -100), (0, 1));

    assert_eq!(turn_wrap_and_count_zeros(50, -68), (82, 1));
    assert_eq!(turn_wrap_and_count_zeros(82, -30), (52, 0));
    assert_eq!(turn_wrap_and_count_zeros(52, 48), (0, 1));
    assert_eq!(turn_wrap_and_count_zeros(16, -16), (0, 1));
    assert_eq!(turn_wrap_and_count_zeros(16, -116), (0, 2));
    assert_eq!(turn_wrap_and_count_zeros(16, -115), (1, 1));
}

#[test]
fn wrap_count_test() {
    assert_eq!(_wrap100_and_count_zeroes(50), (50, 0));
    assert_eq!(_wrap100_and_count_zeroes(-50), (50, 1));
    assert_eq!(_wrap100_and_count_zeroes(100), (0, 1));
    assert_eq!(_wrap100_and_count_zeroes(201), (1, 2));

    assert_eq!(_wrap100_and_count_zeroes(50 - 68), (82, 1));
    // tricky tricky tricky -- it STARTED at 0 so it didn't pass 0 on this rotation. This assert fails:
    // assert_eq!(wrap100_and_count_zeroes(0 - 5), (95, 0));
}

#[allow(dead_code)]
const TEST_INPUTS: &str = "L68
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

#[test]
fn part1_test() {
    assert_eq!(part1(TEST_INPUTS), "3".to_string());
}

#[test]
fn part2_test() {
    assert_eq!(part2(TEST_INPUTS), "6".to_string());
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
