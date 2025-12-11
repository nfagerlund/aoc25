use std::ops::Range;

use anyhow::anyhow;

/// Ignore `{joltage requirements}` and determine the fewest button presses
/// needed to make the lights match the desired pattern.
pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("not implemented"))
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("not implemented"))
}

const _EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

struct Machine {
    light_state: u32,
    desired_lights: u32,
    buttons: Vec<u32>,
    joltage_reqs: Vec<usize>,
}

impl Machine {}

fn my_machine(line: &str) -> Option<Machine> {
    let mut stuff = line.split(' ');
    let lights = stuff.next()?.strip_prefix('[')?.strip_suffix(']')?.chars();
    let mut desired_lights = 0u32;
    for light in lights.filter_map(|c| match c {
        '.' => Some(0_u32),
        '#' => Some(1_u32),
        _ => None,
    }) {
        desired_lights = (desired_lights << 1) + light;
    }
    let mut buttons = Vec::<u32>::new();
    let mut joltage_reqs: Option<Vec<usize>> = None;
    for item in stuff {
        match item.as_bytes()[0] {
            b'(' => {
                // button
                let mut button = 0_u32;
                for position in item
                    .strip_prefix('(')?
                    .strip_suffix(')')?
                    .split(',')
                    .filter_map(|d| d.parse::<u32>().ok())
                {
                    button += 1u32 << position;
                }
                buttons.push(button);
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
        light_state: 0,
        desired_lights,
        buttons,
        joltage_reqs: joltage_reqs?,
    })
}

#[test]
fn part1_test() {
    assert_eq!(part1(_EXAMPLE).expect("should ok"), "7".to_string());
}

#[test]
fn part2_test() {
    assert_eq!(part2(_EXAMPLE).expect("should ok"), "LOL".to_string());
}

/// A lil iterator for doing like... "3 of 0..5" -> [0, 1, 2], [0, 1, 3], [0, 1, 4],
/// [0, 2, 3], [0, 2, 4], [0, 3, 4], [1, 2, 3], [1, 2, 4], [1, 3, 4], [2, 3, 4]
struct CombinateIndicesUnrepeated {
    range: Range<usize>,
    num_elements: usize,
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
            num_elements,
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
        for position in (0..state.len()).rev() {
            let mut val = state[position] + 1;
            if val < beyond_max {
                // we're good! walk back forward and re-set affected
                // positions to their new minimums.
                for i in position..state.len() {
                    state[i] = val;
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
fn combinate_indices_test() {
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
