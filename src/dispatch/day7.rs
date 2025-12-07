use anyhow::anyhow;

/// How many times does the beam split, starting from its origin point?
/// remember they can reconverge.
pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let mut lines = input.lines();
    let s_line = lines.next().ok_or(anyhow!("empty input"))?;
    let mut state = BeamState::initialize(s_line);
    for line in lines {
        state.advance(line);
    }

    Ok(format!("{}", state.split_events))
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("not implemented"))
}

const _EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

#[test]
fn part1_test() {
    assert_eq!(part1(_EXAMPLE).expect("should ok"), "21".to_string());
}

#[test]
fn part2_test() {
    assert_eq!(part2(_EXAMPLE).expect("should ok"), "LOL".to_string());
}

fn is_splitter(byte: u8) -> bool {
    byte == b'^'
}

fn is_beam_start(byte: u8) -> bool {
    byte == b'S'
}

struct BeamState {
    state: Vec<bool>,
    split_events: u64,
    // scratch space
    collisions: Option<Vec<usize>>,
}

impl BeamState {
    fn initialize(s_line: &str) -> Self {
        let mut state = Vec::<bool>::with_capacity(s_line.len());
        state.resize(s_line.len(), false);
        for (i, _) in s_line
            .bytes()
            .enumerate()
            .filter(|(_, b)| is_beam_start(*b))
        {
            state[i] = true;
        }
        Self {
            state,
            split_events: 0,
            collisions: Some(Vec::new()),
        }
    }

    /// vacate this beam, spawn left and right beams.
    fn split(&mut self, position: usize) {
        // I actually want to panic here:
        let left = position.checked_sub(1).expect("walked off the left edge!");
        let right = position + 1;
        if right >= self.state.len() {
            panic!("walked off the right edge!");
        }

        self.state[position] = false;
        self.state[left] = true;
        self.state[right] = true;
    }

    fn advance(&mut self, line: &str) {
        // We need to treat these two vecs with disjoint mutability, and the
        // rust compiler just ain't quite smart enough to handle that yet. So
        // we'll detatch the scratchpad while we're using it.
        let mut collisions = self.collisions.take().unwrap_or_default();
        collisions.clear();

        let b_line = line.as_bytes();
        // find collisions in one pass, apply them in a second pass. Prevents
        // double-counting.
        for (beam_index, _) in self
            .state
            .iter()
            .enumerate()
            .filter(|(_, is_beam)| **is_beam)
        {
            if is_splitter(b_line[beam_index]) {
                collisions.push(beam_index);
            }
        }
        for &collision in collisions.iter() {
            self.split(collision);
            self.split_events += 1;
        }

        // re-shelve it
        self.collisions = Some(collisions);
    }
}
