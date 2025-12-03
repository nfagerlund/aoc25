use anyhow::anyhow;
use std::ops::RangeInclusive;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let sum = input
        .split(',')
        .map(|txt| parse_range(txt).expect("hey what the..."))
        .map(process_range_part1)
        .reduce(|acc, e| acc + e);

    sum.map(|i| format!("{}", i)).ok_or(anyhow!("lol"))
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("ugh??"))
}

#[test]
fn part1_test() {
    assert_eq!(
        part1(_EXAMPLE).expect("should ok"),
        "1227775554".to_string()
    );
}

#[test]
fn part_2_test() {
    assert_eq!(
        part2(_EXAMPLE).expect("should ok"),
        "4174379265".to_string()
    );
}

const _EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
const _EXAMPLE_PER_ITEM_PART1_COUNTS: [u64; 11] = [2, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0];
const _EXAMPLE_PER_ITEM_PART2_COUNTS: [u64; 11] = [2, 2, 2, 1, 1, 0, 1, 1, 1, 1, 1];

// new rules for part two: identify numbers made only of sequences repeated
// _any_ number of times, not just twice. So like 824824824 qualifies.

/// Returns the sum of the repeated sequence numbers within the given range.
fn process_range_part1(r: RangeInclusive<u64>) -> u64 {
    println!("testing {:?}", &r);
    let start = *r.start();
    let end = *r.end();
    let mut repeat_seq = first_repeatable_digit_sequence_from(start);
    let mut sum = 0;
    loop {
        let v = repeat_digits(repeat_seq);
        if r.contains(&v) {
            sum += v;
        } else if v > end {
            // we went past the end of the range. (If we're before the range,
            // like say it started at 1381 and 1313 is too low, keep trying.)
            break;
        }
        repeat_seq += 1;
        println!("  found: {}", v);
    }
    sum
}

fn parse_range(txt: &str) -> Result<RangeInclusive<u64>, anyhow::Error> {
    let (first, second) = txt.split_once('-').ok_or(anyhow!("not hyphenated pair"))?;
    let (start, end) = (first.trim().parse::<u64>()?, second.trim().parse::<u64>()?);

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
fn first_repeatable_digit_sequence_from(start: u64) -> u64 {
    let even = first_even_digited_number_from(start);
    let half_count = digits(even).count() / 2;
    let mut power = (half_count - 1) as u32;
    let mut total = 0_u64;
    for digit in digits(even) {
        total += digit * 10_u64.pow(power);
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
fn first_even_digited_number_from(start: u64) -> u64 {
    let dig_count = digits(start).count();
    // degenerate case: count == 1 but can't ilog10
    if start == 0 {
        10
    } else if dig_count.is_multiple_of(2) {
        start
    } else {
        let next_power = start.ilog10() + 1;
        10u64.pow(next_power)
    }
}

fn is_repeaty(num: u64) -> bool {
    let max_interval = num.div_ceil(2) as u32;
    if max_interval == 0 {
        // can't repeat to produce a single digit
        return false;
    }
    // let mut interval = max_interval;
    'interval: for i in 1..=max_interval {
        let mut d = Digits::new_with_interval(num, i);
        let Some(first_unit) = d.next() else {
            continue;
        };
        // a single bite ain't enough
        if d.len() < 1 {
            continue;
        }
        for next in d {
            if first_unit != next {
                continue 'interval;
            }
        }
        // Oh, looks like they were all the same. That's a repeaty.
        return true;
    }
    false
}

// We need a digits iterator. 10532 -> 1 0 5 3 2
// We can combine .ilog10() and 10.pow() and division and modulus to do this.
// We can do size_hint and exact size iterator to get count of digits.

/// An iterator that takes a number (and an optional number of digits to eat at
/// a time), then returns the _numbers_ that would be formed by removing the
/// specified quantity of digits from the left side of the number. If the number
/// isn't an exact multiple of the requested unit of digits, the final result
/// will be the number formed by whatever digits remain. See tests for more
/// deets, but in short, digits(8007568, 2) => 80, 7, 56, 8.
///
/// As shown in that little example above, do watch your ass around zero-padding.
struct Digits {
    current: u64,
    place_power: u32,
    power_interval: u32,
    dead: bool,
}

fn digits(n: u64) -> Digits {
    Digits::new(n)
}

impl Digits {
    fn new(number: u64) -> Self {
        Self::new_with_interval(number, 1)
    }

    fn new_with_interval(number: u64, interval: u32) -> Self {
        let place_power = if number == 0 {
            0 // ones digit
        } else {
            number.ilog10().saturating_sub(interval - 1)
        };
        Self {
            current: number,
            place_power,
            power_interval: interval,
            dead: false,
        }
    }
}

impl Iterator for Digits {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.dead {
            return None;
        }
        let power = self.place_power;
        let divisor = 10_u64.pow(power);
        let val = self.current / divisor;
        self.current %= divisor; // bottoms out at 0 when run on single digit 👍🏼
        // we're done if: we just handled the ones digit (i.e. for 10^0).
        if power == 0 {
            self.dead = true;
        } else {
            // tee up the next digit
            self.place_power = self.place_power.saturating_sub(self.power_interval);
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
        let remaining = num_digits.div_ceil(self.power_interval);
        (remaining as usize, Some(remaining as usize))
    }
}

impl ExactSizeIterator for Digits {
    // Use all default impls, bc size_hint is 100% reliable
}

// We need a way to repeat a sequence of digits and output a u64.
// I think we can use .ilog10 and 10.pow() and multiplication and addition to do this.

/// Given a u64 that makes a sequence of digits, return a u64 that repeats that
/// sequence twice. so 123 -> 123123. 0 will panic. This does not check for
/// overflow, which might bite me on part 2.
fn repeat_digits(sequence: u64) -> u64 {
    let power = sequence.ilog10() + 1;
    let shifted = sequence * 10_u64.pow(power);
    shifted + sequence
}

#[test]
fn div_ceil_is_neat() {
    assert_eq!(5_u32.div_ceil(2), 3);
    assert_eq!(6_u32.div_ceil(2), 3);
    assert_eq!(0_u32.div_ceil(2), 0);
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
fn is_repeaty_test() {
    assert!(is_repeaty(11));
    assert!(is_repeaty(111));
    assert!(is_repeaty(2424));
    assert!(is_repeaty(242424));
    assert!(is_repeaty(456456456));

    assert!(!is_repeaty(1));
    assert!(!is_repeaty(0));
    assert!(!is_repeaty(24));
    assert!(!is_repeaty(101011));
}

#[test]
fn digits_iterator_test() {
    assert_eq!(Digits::new(4852798).count(), 7);
    assert_eq!(Digits::new(1).count(), 1);
    assert_eq!(Digits::new(0).count(), 1);
    assert_eq!(Digits::new(10).count(), 2);
    assert_eq!(Digits::new(666).count(), 3);
    assert_eq!(Digits::new(1000).count(), 4);
    assert_eq!(Digits::new(4852798).len(), 7);
    assert_eq!(Digits::new(1).len(), 1);
    assert_eq!(Digits::new(0).len(), 1);
    assert_eq!(Digits::new(10).len(), 2);
    assert_eq!(Digits::new(666).len(), 3);
    assert_eq!(Digits::new(1000).len(), 4);

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
fn digits_with_interval_test() {
    // exact multiple number of digits
    assert_eq!(Digits::new_with_interval(245896, 2).len(), 3); // uses size_hint
    assert_eq!(Digits::new_with_interval(245896, 2).count(), 3); // ignores size_hint, burns thru
    let mut d = Digits::new_with_interval(245896, 2);
    assert_eq!(d.next(), Some(24));
    assert_eq!(d.next(), Some(58));
    assert_eq!(d.next(), Some(96));
    assert!(d.next().is_none());

    assert_eq!(Digits::new_with_interval(245896, 3).len(), 2);
    assert_eq!(Digits::new_with_interval(245896, 3).count(), 2);
    let mut d = Digits::new_with_interval(245896, 3);
    assert_eq!(d.next(), Some(245));
    assert_eq!(d.next(), Some(896));
    assert!(d.next().is_none());

    // uneven multiple digits
    assert_eq!(Digits::new_with_interval(3588769, 2).len(), 4);
    assert_eq!(Digits::new_with_interval(3588769, 2).count(), 4);
    let mut d = Digits::new_with_interval(3588769, 2);
    assert_eq!(d.next(), Some(35));
    assert_eq!(d.next(), Some(88));
    assert_eq!(d.next(), Some(76));
    // We return the number corresponding to the remaining digits, even if it's
    // not the full 2 digits
    assert_eq!(d.next(), Some(9));
    assert!(d.next().is_none());

    // With zeroes and shit
    assert_eq!(Digits::new_with_interval(30050, 2).len(), 3);
    assert_eq!(Digits::new_with_interval(30050, 2).count(), 3);
    let mut d = Digits::new_with_interval(30050, 2);
    assert_eq!(d.next(), Some(30));
    assert_eq!(d.next(), Some(5));
    assert_eq!(d.next(), Some(0));
    assert!(d.next().is_none());
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
