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
