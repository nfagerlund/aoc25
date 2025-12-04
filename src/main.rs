mod day1;
mod day2;
mod day3;

/// Maps puzzle identifiers to implementation functions. The main thing you
/// gotta add when starting a new day.
fn puzzle_fn(p: &str) -> fn(&str) -> Result<String, anyhow::Error> {
    match p {
        "1-1" => day1::part1,
        "1-2" => day1::part2,
        "2-1" => day2::part1,
        "2-2" => day2::part2,
        "3-1" => day3::part1,
        "3-2" => day3::part2,
        _ => panic!("That's not a valid puzzle yet"),
    }
}

fn input_string(p: &str) -> String {
    let day = p.split('-').next().expect("actually that can't ever fail.");
    let input_file = format!("inputs/day{day}.txt");
    std::fs::read_to_string(input_file).expect("can't find inputs for that")
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let puzzle = args.next().expect("Requires a puzzle argument, e.g. `1-1`");
    let f = puzzle_fn(&puzzle);
    let input = input_string(&puzzle);
    let output = f(&input).expect("implementation returned error");

    println!("got output:\n{}", &output);
}

#[test]
fn args_test() {
    for a in std::env::args() {
        println!("{}", a);
    }
    // panic!();
}
