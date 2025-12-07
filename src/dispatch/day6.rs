use anyhow::anyhow;
use std::fmt::Display;
use std::iter::Rev;
use std::ops::Add;
use std::ops::Mul;
use std::str::Chars;

/// Solve all the columnar addition-or-multiplication problems, and sum the answers.
pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let mut muncher = LinearProblemMuncher::maybe_new(input).ok_or(anyhow!("not enough lines?"))?;
    let mut storage = Vec::<u64>::new();
    let mut sum = 0_u64;
    while let Some(problem) = muncher.next_problem(storage) {
        println!("{}", &problem);
        sum += problem.solve();
        storage = problem.recycle_storage();
    }
    Ok(format!("{sum}"))
}

/// omfg gotta throw it all away. Now columnar position of digits matters.
/// Derive the *vertically written* numbers and try again.
pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let mut muncher = RTLColumnarProblemMuncher::maybe_new(input)
        .ok_or(anyhow!("Must not have had enough lines??"))?;
    let mut storage = Vec::<u64>::new();
    let mut sum = 0_u64;

    while let Some(problem) = muncher.next_problem(storage) {
        println!("{}", &problem);
        sum += problem.solve();
        storage = problem.recycle_storage();
    }

    Ok(format!("{sum}"))
}

// Looks like they all stay positive. 🤗
// oh wait fuck
// this is so nasty
// END OF LINE SPACES ARE SIGNIFICANT
// EDITOR STRIPS THEM ON SAVE
// FOR-REALSIES INPUTS PROBABLY ALSO CORRUPTED, GOTTA RE-DOWNLOAD
// hate it
const _EXAMPLE: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";

#[test]
fn part1_test() {
    assert_eq!(part1(_EXAMPLE).expect("should ok"), "4277556".to_string());
}

#[test]
fn part2_test() {
    assert_eq!(part2(_EXAMPLE).expect("should ok"), "3263827".to_string());
}

type OpFn = fn(u64, u64) -> u64;

struct Operation {
    op_fn: OpFn,
    name: char,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

fn char_op(in_char: char) -> Option<Operation> {
    let maybe_op_fn: Option<OpFn> = match in_char {
        '+' => Some(u64::add),
        '*' => Some(u64::mul),
        _ => None,
    };
    maybe_op_fn.map(|f| Operation {
        op_fn: f,
        name: in_char,
    })
}

struct Problem {
    // we don't know how many, so we GOTS to vec. or re-implement occupied
    // length knowledge for a fixed array, which no
    numbers: Vec<u64>,
    operation: Operation,
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", &self.operation, &self.numbers)
    }
}

impl Problem {
    fn new(numbers: Vec<u64>, operation: Operation) -> Self {
        Self { numbers, operation }
    }

    /// Consume, clear the storage, and return the already-allocated Vec
    /// for re-use.
    fn recycle_storage(self) -> Vec<u64> {
        let Self {
            mut numbers,
            operation: _,
        } = self;
        numbers.clear();
        numbers
    }

    /// The Answre
    fn solve(&self) -> u64 {
        self.numbers
            .iter()
            .copied()
            .reduce(self.operation.op_fn)
            .unwrap_or(0)
    }
}

// Ok, now..... in some ways, this is actually easier. Item 1, Chars implements
// DoubleEndedIterator. Item 2, if we travel from RTL, the operator always marks
// the final number of the problem.

/// Walk through digit + operator columns in right-to-left direction, extracting
/// math problems. Each column has either a number, a number and an operator
/// (signalling the end of the problem), or nothing but spaces (separating
/// problems).
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

    fn waste_column(&mut self) {
        for digit_feed in self.digit_feeds.iter_mut() {
            digit_feed.next();
        }
        self.operator_feed.next();
    }

    /// This is the place where we guarantee lockstep processing of all our line
    /// iterators. Bails when any feed line runs out.
    fn process_column(&mut self) -> Option<(u64, Option<Operation>)> {
        let mut accum = 0_u64;
        for digit_feed in self.digit_feeds.iter_mut() {
            // Spaces can happen anywhere in the column, just ignore em.
            let digit_char = digit_feed.next()?;
            if let Some(digit) = digit_char.to_digit(10) {
                accum *= 10;
                accum += digit as u64;
            }
        }

        let op_char = self.operator_feed.next()?;
        let maybe_operation = char_op(op_char);
        Some((accum, maybe_operation))
    }

    /// not gonna impl Iterator this time because I want to recycle that vec.
    fn next_problem(&mut self, mut storage: Vec<u64>) -> Option<Problem> {
        // shoulda been wasted already but just to be sure,
        storage.clear();

        // loop til we see an op, then return problem. soon as we hit a none on
        // ANY feed line, bail.
        loop {
            let (next_number, maybe_operation) = self.process_column()?;
            storage.push(next_number);
            if let Some(operation) = maybe_operation {
                // all right, first we need to advance EVERY feed line by a
                // single character to eat the problem-separating space.
                self.waste_column();
                // and then we're done for now.
                return Some(Problem::new(storage, operation));
            }
        }
    }
}

// -------- PART ONE EJECTA GOES BELOW THIS LINE ---------

fn str_op(in_str: &str) -> Option<Operation> {
    in_str.chars().next().and_then(char_op)
}

/// Mostly just does the record-keeping to make sure we chomp each line of
/// tokens in lockstep.
struct LinearProblemMuncher<'a> {
    number_feeds: Vec<SpaceExcavator<'a>>,
    operator_feed: SpaceExcavator<'a>,
}

impl<'a> LinearProblemMuncher<'a> {
    fn maybe_new(input: &'a str) -> Option<Self> {
        let mut stuff: Vec<SpaceExcavator<'a>> = input.lines().map(SpaceExcavator::new).collect();
        let operator_feed = stuff.pop()?;
        if stuff.is_empty() {
            return None;
        }

        Some(Self {
            number_feeds: stuff,
            operator_feed,
        })
    }

    fn next_operator(&mut self) -> Option<Operation> {
        self.operator_feed.next().and_then(str_op)
    }

    fn next_problem(&mut self, mut storage: Vec<u64>) -> Option<Problem> {
        storage.clear();

        for feed in self.number_feeds.iter_mut() {
            let num: u64 = feed.next()?.parse().ok()?;
            storage.push(num);
        }
        let op = self.next_operator()?;
        Some(Problem::new(storage, op))
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
