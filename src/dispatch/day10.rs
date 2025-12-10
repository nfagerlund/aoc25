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
    light_state: Vec<bool>,
    desired_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_reqs: Vec<usize>,
}

impl Machine {}

fn my_machine(line: &str) -> Option<Machine> {
    let mut stuff = line.split(' ');
    let lights = stuff.next()?.strip_prefix('[')?.strip_suffix(']')?.chars();
    let desired_lights = lights
        .map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            _ => None,
        })
        .collect::<Option<Vec<bool>>>()?;
    let mut buttons = Vec::<Vec<usize>>::new();
    let mut joltage_reqs: Option<Vec<usize>> = None;
    for item in stuff {
        match item.as_bytes()[0] {
            b'(' => {
                // button
                let button: Option<Vec<usize>> = item
                    .strip_prefix('(')?
                    .strip_suffix(')')?
                    .split(',')
                    .map(|d| d.parse::<usize>().ok())
                    .collect();
                buttons.push(button?);
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
        light_state: vec![false; desired_lights.len()],
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
