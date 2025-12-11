use std::ops::Range;

use anyhow::anyhow;

#[test]
fn part1_test() {
    assert_eq!(part1(_EXAMPLE).expect("should ok"), "7".to_string());
}

#[test]
fn part2_test() {
    assert_eq!(part2(_EXAMPLE).expect("should ok"), "33".to_string());
}

/// Ignore `{joltage requirements}` and determine the fewest button presses
/// needed to make the lights match the desired pattern.
pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let mut grand_total = 0_usize;
    for machine in input.lines().filter_map(my_machine) {
        grand_total += machine.brute_force_lights_button_counts()?;
    }

    Ok(format!("{grand_total}"))
}

/// OK, now multiple button presses matter. Find minimum # of button presses to
/// make the joltage levels match the joltage requirements. YIKES. Is bruting
/// this out of the question?
pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("not implemented"))
}

const _EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

fn bitbutton(lights_activated: impl Iterator<Item = u32>) -> u32 {
    let mut res = 0_u32;
    for position in lights_activated {
        res += 1_u32 << position;
    }
    res
}

#[test]
fn bitbutton_test() {
    let test = |slice: &[u32], bits: u32| {
        assert_eq!(bitbutton(slice.iter().copied()), bits);
    };
    test(&[3], 0b1000);
    test(&[1, 3], 0b1010);
    test(&[0, 2, 3, 4], 0b11101);
}

/// A bitlight has to be assembled in REVERSE binary place-order! Because the
/// leftmost listed light has to correspond to a named light position of 0
/// (rightmost in a binary number).
fn bitlights(l_str: &str) -> anyhow::Result<u32> {
    let mut res = 0_u32;
    for (i, c) in l_str.chars().enumerate() {
        let bit = match c {
            '.' => 0u32,
            '#' => 1u32,
            _ => {
                return Err(anyhow!("parse err for bit char"));
            }
        };
        res += bit << i;
    }
    Ok(res)
}

#[test]
fn bitlights_test() {
    // remember we put our thing down flipped it and reversed it
    assert_eq!(bitlights(".##.").unwrap(), 0b0110);
    assert_eq!(bitlights("...#.").unwrap(), 0b01000);
    assert_eq!(bitlights(".###.#").unwrap(), 0b101110);
}

/// Okay, listen, this is incredibly cursed, but
/// https://web.archive.org/web/20151229003112/http://blogs.msdn.com/b/jeuge/archive/2005/06/08/hakmem-bit-count.aspx
fn bitcount_u32(u: u32) -> u32 {
    let u_count = u - ((u >> 1) & 0o33333333333) - ((u >> 2) & 0o11111111111);
    ((u_count + (u_count >> 3)) & 0o30707070707) % 63
}

#[test]
fn bitcount_test() {
    let test = |val: u32, num_ones: u32| {
        assert_eq!(bitcount_u32(val), num_ones);
    };
    test(0b10101010, 4);
    test(0b00111, 3);
    test(0b100000110, 3);
    test(u32::MAX, 32);
}

#[derive(Debug)]
struct Machine {
    desired_lights: u32,
    buttons: Vec<u32>,
    joltage_reqs: Vec<usize>,
}

impl Machine {
    /// Okay, so this is a variant on Lights Out. I do NOT know enough linear
    /// algebra to solve it analytically. But I DO know that 1. non-same button
    /// presses are commutative, and 2. pressing the same button twice is the
    /// same as never pressing it. That's enough to be able to beast it. I think.
    fn brute_force_lights_button_counts(&self) -> anyhow::Result<usize> {
        // iter all combinations in increasing magnitude, and early exit when successful.
        for num_presses in 1..=(self.buttons.len()) {
            let combinations =
                CombinateIndicesUnrepeated::try_new(0..self.buttons.len(), num_presses)?;
            for index_list in combinations {
                let mut result = 0_u32;
                for i in index_list {
                    result ^= self.buttons[i];
                }
                if result == self.desired_lights {
                    return Ok(num_presses);
                }
            }
        }

        // Oh... we didn't get anywhere.
        Err(anyhow!("Somehow wasn't able to solve it: {:?}", self))
    }
}

/// (quiet guitar, increasing tension)
fn my_machine(line: &str) -> Option<Machine> {
    let mut stuff = line.split(' ');
    let lights = stuff.next()?.strip_prefix('[')?.strip_suffix(']')?;
    let desired_lights = bitlights(lights).unwrap();
    let mut buttons = Vec::<u32>::new();
    let mut joltage_reqs: Option<Vec<usize>> = None;
    for item in stuff {
        match item.as_bytes()[0] {
            b'(' => {
                // button
                let positions = item
                    .strip_prefix('(')?
                    .strip_suffix(')')?
                    .split(',')
                    .map(|d| d.parse::<u32>().expect("come on, man!!!"));
                buttons.push(bitbutton(positions));
            }
            b'{' => {
                joltage_reqs = item
                    .strip_prefix('{')?
                    .strip_suffix('}')?
                    .split(',')
                    .map(|d| d.parse::<usize>().ok())
                    .collect();
            }
            _ => {
                return None;
            }
        }
    }

    Some(Machine {
        desired_lights,
        buttons,
        joltage_reqs: joltage_reqs?,
    })
}

/// A lil iterator for doing like... "3 of 0..5" -> [0, 1, 2], [0, 1, 3], [0, 1, 4],
/// [0, 2, 3], [0, 2, 4], [0, 3, 4], [1, 2, 3], [1, 2, 4], [1, 3, 4], [2, 3, 4]
struct CombinateIndicesUnrepeated {
    range: Range<usize>,
    // state also encodes the number of elements, in its length.
    state: Option<Vec<usize>>, // None for done
}

fn r_idx(range: &Range<usize>, i: usize) -> Option<usize> {
    let n = range.start + i;
    if n < range.end { Some(n) } else { None }
}

impl CombinateIndicesUnrepeated {
    fn try_new(range: Range<usize>, num_elements: usize) -> anyhow::Result<Self> {
        if num_elements > range.len() {
            return Err(anyhow!(
                "Can't combine {} elements of {} options",
                num_elements,
                range.len()
            ));
        }
        // build initial state, with the first value we'll return
        let mut state = Vec::<usize>::with_capacity(num_elements);
        for n in 0..num_elements {
            let i = r_idx(&range, n).ok_or(anyhow!("impossible, we just checked"))?;
            state.push(i);
        }

        Ok(Self {
            range,
            state: Some(state),
        })
    }
}

impl Iterator for CombinateIndicesUnrepeated {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        // max goes down with every leftward step through the state.
        let mut beyond_max = self.range.end;
        let Some(ref mut state) = self.state else {
            // done.
            return None;
        };
        // Grab our result,
        let res = state.clone();
        // then advance the state. Walk backwards, trying to bump the highest
        // position we can get away with.
        let state_len = state.len();
        for position in (0..state_len).rev() {
            let mut val = state[position] + 1;
            if val < beyond_max {
                // we're good! walk back forward and re-set affected
                // positions to their new minimums.
                for slot in &mut state[position..state_len] {
                    *slot = val;
                    val += 1;
                }
                return Some(res);
            }
            // we're not good! keep walking backward... and decrement our max.
            beyond_max -= 1;
        }
        // If we managed to escape the for-loop without early returning... I
        // think we're out of headroom, i.e. our initial state before trying
        // this woulda been, e.g., [2, 3, 4] in the "3 of 0..5" case. So burn the ships.
        self.state = None;
        // and return our last value.
        Some(res)
    }
}

#[test]
fn combinate_indices_unrepeating_test() {
    let thing = CombinateIndicesUnrepeated::try_new(0..5, 3).unwrap();
    let expect: Vec<Vec<usize>> = vec![
        vec![0, 1, 2],
        vec![0, 1, 3],
        vec![0, 1, 4],
        vec![0, 2, 3],
        vec![0, 2, 4],
        vec![0, 3, 4],
        vec![1, 2, 3],
        vec![1, 2, 4],
        vec![1, 3, 4],
        vec![2, 3, 4],
    ];
    let outcome: Vec<_> = thing.collect();
    assert_eq!(outcome, expect);

    let small = CombinateIndicesUnrepeated::try_new(0..5, 5).unwrap();
    let expect: Vec<Vec<usize>> = vec![vec![0, 1, 2, 3, 4]];
    let outcome: Vec<_> = small.collect();
    assert_eq!(outcome, expect);

    let wrong = CombinateIndicesUnrepeated::try_new(0..5, 6);
    assert!(wrong.is_err());
}

/// Ok, so this one would turn 3 of 0..5 into uhhhhh
/// 000, 001, 002, 003, 004,
/// 011, 012, 013, 014,
/// 022, 023, 024,
/// 033, 034,
/// 044,
/// and, like the pattern we've been seeing, 110 is equivalent to 011, so,
/// 111, 112, 113, 114,
/// 122, 123, 124,
/// 133, 134,
/// 144,
/// 222, 223, 224,
/// 233, 234,
/// 244,
/// 333, 334,
/// 344,
/// 444
struct CombinateIndicesRepeated {
    range: Range<usize>,
    state: Option<Vec<usize>>,
}

impl CombinateIndicesRepeated {
    fn new(range: Range<usize>, num_elements: usize) -> Self {
        // Oddly, there's no error state here. 6 of 0..5 could be [0,0,0,0,0,0].

        // build initial state, with the first value we'll return
        let state = vec![0_usize; num_elements];

        Self {
            range,
            state: Some(state),
        }
    }
}

impl Iterator for CombinateIndicesRepeated {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        // max goes down with every leftward step through the state.
        let beyond_max = self.range.end;
        let Some(ref mut state) = self.state else {
            // done.
            return None;
        };
        // Grab our result,
        let res = state.clone();
        // then advance the state. Walk backwards, trying to bump the highest
        // position we can get away with.
        let state_len = state.len();
        for position in (0..state_len).rev() {
            let val = state[position] + 1;
            if val < beyond_max {
                // we're good! walk back forward and re-set affected
                // positions to their new minimums.
                // THIS IS THE DIFFERENCE: the new forward minimum doesn't
                // increase by position.
                for slot in &mut state[position..state_len] {
                    *slot = val;
                    // val += 1;
                }
                return Some(res);
            }
            // we're not good! keep walking backward...
            // ALSO I think we don't decrement our max this time.
            // beyond_max -= 1;
        }
        // If we managed to escape the for-loop without early returning... I
        // think we're out of headroom, i.e. our initial state before trying
        // this woulda been, e.g., [4, 4, 4] in the "3 of 0..5" case. So burn the ships.
        self.state = None;
        // and return our last value.
        Some(res)
    }
}

#[test]
fn combinate_indices_repeating_test() {
    let thing = CombinateIndicesRepeated::new(0..5, 3);
    let expect: Vec<Vec<usize>> = vec![
        vec![0, 0, 0],
        vec![0, 0, 1],
        vec![0, 0, 2],
        vec![0, 0, 3],
        vec![0, 0, 4],
        vec![0, 1, 1],
        vec![0, 1, 2],
        vec![0, 1, 3],
        vec![0, 1, 4],
        vec![0, 2, 2],
        vec![0, 2, 3],
        vec![0, 2, 4],
        vec![0, 3, 3],
        vec![0, 3, 4],
        vec![0, 4, 4],
        vec![1, 1, 1],
        vec![1, 1, 2],
        vec![1, 1, 3],
        vec![1, 1, 4],
        vec![1, 2, 2],
        vec![1, 2, 3],
        vec![1, 2, 4],
        vec![1, 3, 3],
        vec![1, 3, 4],
        vec![1, 4, 4],
        vec![2, 2, 2],
        vec![2, 2, 3],
        vec![2, 2, 4],
        vec![2, 3, 3],
        vec![2, 3, 4],
        vec![2, 4, 4],
        vec![3, 3, 3],
        vec![3, 3, 4],
        vec![3, 4, 4],
        vec![4, 4, 4],
    ];
    let outcome: Vec<_> = thing.collect();
    assert_eq!(outcome, expect);

    let small = CombinateIndicesRepeated::new(0..1, 5);
    let expect: Vec<Vec<usize>> = vec![vec![0, 0, 0, 0, 0]];
    let outcome: Vec<_> = small.collect();
    assert_eq!(outcome, expect);
}
