use crate::util::Grid;

/// Find all *occupied* cells where fewer than four of the eight surrounding
/// cells are occupied.
pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let grid = build_grid(input)?;
    let all_roll_indices = grid.iter_occupied_indices();
    let all_roll_neighbor_counts = all_roll_indices.map(|i| {
        let coords = grid.coords(i);
        grid.count_occupied_neighbors(coords)
    });
    let accessible_rolls_count = all_roll_neighbor_counts.filter(|c| *c < 4).count();
    Ok(format!("{accessible_rolls_count}"))
}

/// Iteratively count how many rolls of paper can be removed, according to the
/// accessibility rules in part 1. Each iteration exposes more rolls.
pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let mut grid = build_grid(input)?;
    let mut count = 0;

    loop {
        let iteration_count = grid.evict_and_count();
        if iteration_count == 0 {
            break;
        }
        count += iteration_count;
    }

    Ok(format!("{count}"))
}

const _EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

#[test]
fn part1_test() {
    assert_eq!(part1(_EXAMPLE).expect("should ok"), "13".to_string());
}

#[test]
fn part2_test() {
    assert_eq!(part2(_EXAMPLE).expect("should ok"), "43".to_string());
}

fn build_grid(ascii: &str) -> anyhow::Result<Grid<bool>> {
    let width = width_of_ascii_grid(ascii);
    let stuff = load_ascii_grid_to_vec_of_bools(ascii);
    Grid::try_new(width, stuff)
}

fn load_ascii_grid_to_vec_of_bools(input: &str) -> Vec<bool> {
    let stuff = input.bytes().filter_map(|b| {
        match b {
            b'\n' => None,
            b'@' => Some(true),
            b'.' => Some(false),
            _ => None, // shouldn't happen
        }
    });
    Vec::from_iter(stuff)
}

fn width_of_ascii_grid(input: &str) -> usize {
    input
        .lines()
        .next()
        .expect("empty string, just gonna panic early")
        .len()
}

#[test]
fn width_test() {
    assert_eq!(width_of_ascii_grid(_EXAMPLE), 10);
    // assert_eq!(width_of_ascii_grid(""), 0); // nope
    assert_eq!(width_of_ascii_grid("\n"), 0);
}

#[test]
fn byte_char_test() {
    let s = "
hey";
    assert_eq!(s.bytes().next(), Some(b'\n'));
}
