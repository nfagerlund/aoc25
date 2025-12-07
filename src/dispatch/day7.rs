use anyhow::anyhow;

/// How many times does the beam split, starting from its origin point?
/// remember they can reconverge.
pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let state = shared_impl(input)?;
    Ok(format!("{}", state.split_events))
}

fn shared_impl(input: &str) -> anyhow::Result<BeamState> {
    let mut lines = input.lines();
    let s_line = lines.next().ok_or(anyhow!("empty input"))?;
    let mut state = BeamState::initialize(s_line);
    for line in lines {
        state.advance(line);
    }
    Ok(state)
}

/// How many paths could a single particle take through the forest of splitters?
/// uhhhhh...
///
/// So, if we take the first three rows with splitters from the example as the
/// whole deal, we get 8. So, what would be overlapping beams keep getting
/// tracked separately -- it's six exits from the third row, but the middle
/// splitter's outputs count twice.
///
/// If we add the fourth row... is that 13?? yeah. So, I think it's almost like
/// overlapping beams have *more weight.* Let's go with that and see.
pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let state = shared_impl(input)?;
    Ok(format!("{}", state.total_world_lines()))
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
    assert_eq!(part2(_EXAMPLE).expect("should ok"), "40".to_string());
}

fn is_splitter(byte: u8) -> bool {
    byte == b'^'
}

fn is_beam_start(byte: u8) -> bool {
    byte == b'S'
}

struct BeamState {
    /// For each lane of the manifold, the number of paths a beam could have
    /// taken to get there, given the split events recorded so far. For part 1,
    /// we only care about > 0, but part 2 wants totals.
    state: Vec<u64>,
    /// The number of unique splitters that the beam has collided with, without
    /// regard for how many possible paths the beam could have taken to get to
    /// each splitter. Part 1 wants this but part 2 doesn't care.
    split_events: u64,
    // scratch space
    collisions: Option<Vec<usize>>,
}

impl BeamState {
    fn initialize(s_line: &str) -> Self {
        let mut state: Vec<u64> = vec![0; s_line.len()];
        for (i, _) in s_line
            .bytes()
            .enumerate()
            .filter(|(_, b)| is_beam_start(*b))
        {
            state[i] = 1;
        }
        Self {
            state,
            split_events: 0,
            collisions: Some(Vec::new()),
        }
    }

    /// vacate this lane, spawn left and right beams. hitting a splitter doubles
    /// the number of paths that pass through it; if there were six paths leading to
    /// this splitter, then twelve lead out of it.
    fn split(&mut self, position: usize) {
        // I actually want to panic here:
        let left = position.checked_sub(1).expect("walked off the left edge!");
        let right = position + 1;
        if right >= self.state.len() {
            panic!("walked off the right edge!");
        }

        let routes = self.state[position];
        self.state[position] = 0;
        self.state[left] += routes;
        self.state[right] += routes;
    }

    fn total_world_lines(&self) -> u64 {
        self.state.iter().sum()
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
            .filter(|(_, routes)| **routes > 0)
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
