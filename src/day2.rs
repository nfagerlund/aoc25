use anyhow::anyhow;
use std::ops::RangeInclusive;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let sum = input
        .split(',')
        .filter_map(|txt| parse_range(txt).ok())
        .map(process_range_part1)
        .reduce(|acc, e| acc + e);

    sum.map(|i| format!("{}", i)).ok_or(anyhow!("lol"))
}

#[test]
fn part1_test() {
    assert_eq!(
        part1(_EXAMPLE).expect("should ok"),
        "1227775554".to_string()
    );
}

const _EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
const _EXAMPLE_PER_ITEM_PART1_COUNTS: [u32; 11] = [2, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0];

/// Returns the sum of the repeated sequence numbers within the given range.
fn process_range_part1(r: RangeInclusive<u32>) -> u32 {
    let start = *r.start();
    let mut repeat_seq = first_repeatable_digit_sequence_from(start);
    let mut sum = 0;
    loop {
        let v = repeat_digits(repeat_seq);
        // Have we gone past the end of the range yet?
        if !r.contains(&v) {
            break;
        }
        sum += v;
        repeat_seq += 1;
    }
    sum
}

fn parse_range(txt: &str) -> Result<RangeInclusive<u32>, anyhow::Error> {
    let (first, second) = txt.split_once('-').ok_or(anyhow!("not hyphenated pair"))?;
    let (start, end) = (first.parse::<u32>()?, second.parse::<u32>()?);

    Ok(start..=end)
}

// ok...... so we're looking for exactly 2x repeated sequences of digits. like 123123.
// - Must be even number of digits.
// - ...identify the first half, check if the range includes second half, increment it,
// check if range includes second half... continue...?
//
// * 11 -> [1]1 -> range.includes(11)? -> increment: [2] -> range.includes(22)? -> increment: [3]
// -> range.includes(33)? NO.
// * 998 -> odd. proceed to even. -> 1000 -> [10]00 -> range.includes(1010)? -> increment: [11]
// -> range.includes(1111)? NO.
// * ...

// We need a way to use the digits iterator to extract a repeatable sequence
// from the first half of a number.
// Say we have eight digits, 54298345. We want 5429. We get the half-count (4).
// We take first digit (5) and multiply it by 10^3, then add 4 * 10^2...
fn first_repeatable_digit_sequence_from(start: u32) -> u32 {
    let even = first_even_digited_number_from(start);
    let half_count = digits(even).count() / 2;
    let mut power = (half_count - 1) as u32;
    let mut total = 0_u32;
    for digit in digits(even) {
        total += digit * 10_u32.pow(power);
        if power == 0 {
            // that was the ones.
            break;
        }
        power -= 1;
    }
    total
}

// We need a way to find the next even-digited number, given a starting number.
// * if n % 2 == 0, n.
// * ...say we have 301... we want 1000. 301.ilog10() = 2, 10^2 = 100, so we want 10^3
// * else, 10.pow(n.ilog10() + 1)

/// Like the name says. But 0 returns 0, so take note.
fn first_even_digited_number_from(start: u32) -> u32 {
    let dig_count = digits(start).count();
    // degenerate case: count == 1 but can't ilog10
    if start == 0 {
        10
    } else if dig_count.is_multiple_of(2) {
        start
    } else {
        let next_power = start.ilog10() + 1;
        10u32.pow(next_power)
    }
}

// We need a digits iterator. 10532 -> 1 0 5 3 2
// We can combine .ilog10() and 10.pow() and division and modulus to do this.
// We can do size_hint and exact size iterator to get count of digits.

/// An iterator that returns the digits of a u32 (base10) as u32s. As the
/// degenerate case, we treat 0 as having zero digits, even though that's strange.
struct Digits {
    current: u32,
    place_power: u32,
    dead: bool,
}

fn digits(n: u32) -> Digits {
    Digits::new(n)
}

impl Digits {
    fn new(current: u32) -> Self {
        let place_power = if current == 0 {
            0 // ones digit
        } else {
            current.ilog10()
        };
        Self {
            current,
            place_power,
            dead: false,
        }
    }
}

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.dead {
            return None;
        }
        let power = self.place_power;
        let divisor = 10_u32.pow(power);
        let val = self.current / divisor;
        self.current %= divisor; // bottoms out at 0 when run on single digit 👍🏼
        // we're done if: we just handled the ones digit (i.e. for 10^0).
        if power == 0 {
            self.dead = true;
        } else {
            // tee up the next digit
            self.place_power -= 1;
        }
        Some(val)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.dead {
            return (0, Some(0));
        }
        // degenerate case can't ilog10 but still valid...
        if self.current == 0 {
            return (1, Some(1));
        }
        // 10 ^ 0 = 1, 10 ^ 1 = 10...
        let num_digits = self.current.ilog10() + 1;
        (num_digits as usize, Some(num_digits as usize))
    }
}

impl ExactSizeIterator for Digits {
    // Use all default impls, bc size_hint is 100% reliable
}

// We need a way to repeat a sequence of digits and output a u32.
// I think we can use .ilog10 and 10.pow() and multiplication and addition to do this.

/// Given a u32 that makes a sequence of digits, return a u32 that repeats that
/// sequence twice. so 123 -> 123123. 0 will panic. This does not check for
/// overflow, which might bite me on part 2.
fn repeat_digits(sequence: u32) -> u32 {
    let power = sequence.ilog10() + 1;
    let shifted = sequence * 10_u32.pow(power);
    shifted + sequence
}

#[test]
fn first_even_from_test() {
    assert_eq!(first_even_digited_number_from(0), 10);
    assert_eq!(first_even_digited_number_from(1), 10);
    assert_eq!(first_even_digited_number_from(10), 10);
    assert_eq!(first_even_digited_number_from(11), 11);
    assert_eq!(first_even_digited_number_from(81), 81);
    assert_eq!(first_even_digited_number_from(230), 1000);
    assert_eq!(first_even_digited_number_from(1000), 1000);
    assert_eq!(first_even_digited_number_from(2153), 2153);
    assert_eq!(first_even_digited_number_from(55555), 100000);
}

#[test]
fn first_repeatable_from_test() {
    // 0 -> 10 -> 1
    assert_eq!(first_repeatable_digit_sequence_from(0), 1);
    // 1 -> 10 -> 1
    assert_eq!(first_repeatable_digit_sequence_from(1), 1);
    // 9 -> 10 -> 1
    assert_eq!(first_repeatable_digit_sequence_from(9), 1);
    assert_eq!(first_repeatable_digit_sequence_from(10), 1);
    assert_eq!(first_repeatable_digit_sequence_from(11), 1);
    assert_eq!(first_repeatable_digit_sequence_from(12), 1);
    assert_eq!(first_repeatable_digit_sequence_from(20), 2);
    assert_eq!(first_repeatable_digit_sequence_from(24), 2);
    // xxx -> 1000 -> 10
    assert_eq!(first_repeatable_digit_sequence_from(666), 10);
    assert_eq!(first_repeatable_digit_sequence_from(2456), 24);
    // xxxxx -> 100000 -> 100
    assert_eq!(first_repeatable_digit_sequence_from(57382), 100);
}

#[test]
fn repeat_digits_test() {
    assert_eq!(repeat_digits(123), 123123);
    assert_eq!(repeat_digits(1), 11);
    assert_eq!(repeat_digits(22), 2222);
    // 0 just panics. ok, so don't do that then.
    // assert_eq!(repeat_digits(0), 0);
}

#[test]
fn digits_iterator_test() {
    assert_eq!(Digits::new(4852798).count(), 7);
    assert_eq!(Digits::new(1).count(), 1);
    assert_eq!(Digits::new(0).count(), 1);
    assert_eq!(Digits::new(10).count(), 2);
    assert_eq!(Digits::new(666).count(), 3);
    assert_eq!(Digits::new(1000).count(), 4);

    let mut d = Digits::new(4852798);
    assert_eq!(d.next(), Some(4));
    assert_eq!(d.next(), Some(8));
    assert_eq!(d.next(), Some(5));
    assert_eq!(d.next(), Some(2));
    assert_eq!(d.next(), Some(7));
    assert_eq!(d.next(), Some(9));
    assert_eq!(d.next(), Some(8));
    assert_eq!(d.next(), None);
    assert_eq!(d.next(), None);
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
