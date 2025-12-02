use anyhow::anyhow;
use std::ops::RangeInclusive;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("lol"))
}

const _EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
const _EXAMPLE_PER_ITEM_PART1_COUNTS: [u32; 11] = [2, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0];

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
//

// We need a digits iterator. 10532 -> 1 0 5 3 2
// We can combine .ilog10() and 10.pow() and division and modulus to do this.
// We can do size_hint and exact size iterator to get count of digits.

/// An iterator that returns the digits of a u32 (base10) as u32s. As the
/// degenerate case, we treat 0 as having zero digits, even though that's strange.
struct Digits {
    current: u32,
    dead: bool,
}

fn digits(n: u32) -> Digits {
    Digits::new(n)
}

impl Digits {
    fn new(current: u32) -> Self {
        Self {
            current,
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
        // degenerate case can't ilog10, also we're done here
        if self.current == 0 {
            self.dead = true;
            return Some(0);
        }
        let power = self.current.ilog10();
        let divisor = 10_u32.pow(power);
        let val = self.current / divisor;
        self.current %= divisor; // bottoms out at 0 when run on single digit 👍🏼
        // we're done if: we just handled the ones digit (i.e. for 10^0).
        if power == 0 {
            self.dead = true;
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
