use anyhow::anyhow;
use std::iter::Rev;
use std::ops::Add;
use std::ops::Mul;
use std::str::Chars;

/// Solve all the columnar addition-or-multiplication problems, and sum the answers.
pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let muncher = LinearProblemMuncher::try_new(input)?;
    let sum = muncher.reduce(u64::add).expect("wait why's it empty");
    Ok(format!("{sum}"))
}

/// omfg gotta throw it all away. Now columnar position of digits matters.
/// Derive the *vertically written* numbers and try again.
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
    assert_eq!(part2(_EXAMPLE).expect("should ok"), "3263827".to_string());
}

type Op = fn(u64, u64) -> u64;

fn str_op(in_str: &str) -> anyhow::Result<Op> {
    match in_str {
        "+" => Ok(u64::add),
        "*" => Ok(u64::mul),
        _ => Err(anyhow!("Unrecognized operation {in_str}")),
    }
}

fn char_op(in_char: char) -> Option<Op> {
    match in_char {
        '+' => Some(u64::add),
        '*' => Some(u64::mul),
        _ => None,
    }
}

struct Problem {
    // we don't know how many, so we GOTS to vec. or re-implement occupied
    // length knowledge for a fixed array, which no
    numbers: Vec<u64>,
    operator: Op,
}

impl Problem {
    fn new(numbers: Vec<u64>, operator: Op) -> Self {
        Self { numbers, operator }
    }

    /// Consume, clear the storage, and return the already-allocated Vec
    /// for re-use.
    fn recycle_storage(self) -> Vec<u64> {
        let Self {
            mut numbers,
            operator: _,
        } = self;
        numbers.clear();
        numbers
    }

    /// The Answre
    fn solve(&self) -> u64 {
        self.numbers
            .iter()
            .copied()
            .reduce(self.operator)
            .unwrap_or(0)
    }
}

// Ok, now..... in some ways, this is actually easier. Item 1, Chars implements
// DoubleEndedIterator. Item 2, if we travel from RTL, the operator always marks
// the final number of the problem.

/// Panics on hard parse errors bc it's a big ol baby (i.e. i'd rather get clear
/// signals fast and this doesn't have to be resilient, so let's keep our type
/// signatures clean).
struct RTLColumnarProblemMuncher<'a> {
    digit_feeds: Vec<Rev<Chars<'a>>>,
    operator_feed: Rev<Chars<'a>>,
}

impl<'a> RTLColumnarProblemMuncher<'a> {
    fn maybe_new(input: &'a str) -> Option<Self> {
        let mut stuff: Vec<Rev<Chars<'a>>> = input.lines().map(|l| l.chars().rev()).collect();
        let operator_feed = stuff.pop()?;
        if stuff.is_empty() {
            return None;
        }

        Some(Self {
            digit_feeds: stuff,
            operator_feed,
        })
    }

    fn waste_a_column(&mut self) {
        for digit_feed in self.digit_feeds.iter_mut() {
            digit_feed.next();
        }
        self.operator_feed.next();
    }

    /// DANGER: this ain't pub, don't call it. (I don't feel like extracting
    /// this to a separate mod rn.)
    /// Bails when at least one feed line is empty.
    fn extract_number(&mut self) -> Option<u64> {
        let mut accum = 0_u64;
        for digit_feed in self.digit_feeds.iter_mut() {
            // Spaces can happen anywhere in the column, just ignore em.
            if let Some(digit) = digit_feed.next()?.to_digit(10) {
                accum *= 10;
                accum += digit as u64;
            }
        }
        Some(accum)
    }

    /// not gonna impl Iterator this time because I want to recycle that vec.
    fn next_problem(&mut self, mut storage: Vec<u64>) -> Option<Problem> {
        // shoulda been wasted already but just to be sure,
        storage.clear();

        // loop til we see an op, then return problem. soon as we hit a none on
        // ANY feed line, bail.
        loop {
            let next_number = self.extract_number()?;
            let maybe_operator = self.operator_feed.next()?;
            storage.push(next_number);
            if let Some(operator) = char_op(maybe_operator) {
                // all right, first we need to advance EVERY feed line by a
                // single character to eat the problem-separating space.
                self.waste_a_column();
                // and then we're done for now.
                return Some(Problem {
                    numbers: storage,
                    operator,
                });
            }
        }
    }
}

// -------- PART ONE EJECTA GOES BELOW THIS LINE ---------

/// Mostly just does the record-keeping to make sure we chomp each line of
/// tokens in lockstep.
struct LinearProblemMuncher<'a> {
    storage: Vec<SpaceExcavator<'a>>,
}

impl<'a> LinearProblemMuncher<'a> {
    // Bounds check here lets us ignore hella-malformed input later.
    fn try_new(input: &'a str) -> anyhow::Result<Self> {
        let storage: Vec<_> = input.lines().map(SpaceExcavator::new).collect();
        if storage.len() < 2 {
            Err(anyhow!(
                "Malformed input: we need at least one numbers line and an operators line."
            ))
        } else {
            Ok(Self { storage })
        }
    }

    fn operator_feed(&mut self) -> &mut SpaceExcavator<'a> {
        let len = self.storage.len();
        &mut self.storage[len - 1]
    }

    fn number_feeds(&mut self) -> &mut [SpaceExcavator<'a>] {
        let len = self.storage.len();
        &mut self.storage[0..(len - 1)]
    }

    // Hmm, at this point we gotta start translating from result to option... I
    // think I'll just panic instead 👍🏼👍🏼👍🏼
    fn next_operator(&mut self) -> Option<Op> {
        let op_str = self.operator_feed().next()?;
        Some(str_op(op_str).unwrap())
    }

    // eh........ in this case I guess I'll just swallow parse errors as
    // nones..... hate it slightly. Oh, by the way, you must consume the whole
    // iterator here or else everything kinda goes to hell. I don't feel like
    // making a new type for that and implementing drop. all told, this experiment
    // has been a very intensely qualified success.
    fn dangerous_next_numbers_iter(&mut self) -> impl Iterator<Item = u64> {
        self.number_feeds()
            .iter_mut()
            .filter_map(|feed| feed.next().and_then(|s| s.parse::<u64>().ok()))
    }

    /// Never call dangerous_next_numbers_iter or next_operator, always call
    /// this instead.
    fn next_solution(&mut self) -> Option<u64> {
        let op = self.next_operator()?;
        self.dangerous_next_numbers_iter().reduce(op)
    }
}

impl<'a> Iterator for LinearProblemMuncher<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_solution()
    }
}

/// An iterator to split space-separated strings where the number of spaces
/// might vary. I'm being extra to avoid having to pull in the regexp crate
/// until I can't avoid it.
///
/// This only handles the literal ascii space character; it doesn't know about
/// tabs or exotic spaces (👾👽) or newlines.
struct SpaceExcavator<'a> {
    slice: &'a str,
    last_index: usize,
}

impl<'a> SpaceExcavator<'a> {
    fn new(slice: &'a str) -> Self {
        Self {
            slice,
            last_index: 0,
        }
    }
}
impl<'a> Iterator for SpaceExcavator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // Advance past any spaces, mark our start, advance past non-spaces,
        // mark our end, update the last index, and return the non-space region
        // we traversed.
        let mut start = self.last_index;

        // can't index into str directly, so:
        let bytes = self.slice.as_bytes();
        let len = bytes.len();
        while start < len && bytes[start] == b' ' {
            start += 1;
        }
        // we dead?
        if start >= len {
            self.last_index = start;
            return None;
        }
        // ok.

        let mut end = start;
        while end < len && bytes[end] != b' ' {
            end += 1;
        }
        self.last_index = end;

        // This theoretically can panic, but we just did math so it's fine. :]
        Some(&self.slice[start..end])
    }
}

#[test]
fn space_excavator_test() {
    let testcases = vec![
        (
            SpaceExcavator::new("  leading and  trailing   spaces   "),
            vec!["leading", "and", "trailing", "spaces"],
        ),
        (
            SpaceExcavator::new("only  interior     spaces"),
            vec!["only", "interior", "spaces"],
        ),
    ];
    for (mut iter, words) in testcases {
        for word in words {
            assert_eq!(iter.next(), Some(word));
        }
        assert!(iter.next().is_none());
        assert_eq!(iter.last_index, iter.slice.len());
    }
}
