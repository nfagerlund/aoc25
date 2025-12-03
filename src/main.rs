mod day1;
mod day2;
mod day3;

fn main() {
    let mut args = std::env::args();
    args.next();
    let puzzle = args.next().expect("Requires a puzzle argument, e.g. `1-1`");
    let output = match puzzle.as_str() {
        "1-1" => {
            let input =
                std::fs::read_to_string("inputs/1.txt").expect("can't find inputs for day 1");
            day1::part1(&input)
        }
        "1-2" => {
            let input =
                std::fs::read_to_string("inputs/1.txt").expect("can't find inputs for day 1");
            day1::part2(&input)
        }
        "2-1" => {
            let input = std::fs::read_to_string("inputs/day2.txt").expect("can't find");
            day2::part1(&input).expect("should ok")
        }
        "2-2" => {
            let input = std::fs::read_to_string("inputs/day2.txt").expect("can't find");
            day2::part2(&input).expect("should ok")
        }
        "3-1" => {
            let input = std::fs::read_to_string("inputs/day3.txt").expect("can't find");
            day3::part1(&input).expect("should ok")
        }
        "3-2" => {
            let input = std::fs::read_to_string("inputs/day3.txt").expect("can't find");
            day3::part2(&input).expect("should ok")
        }
        _ => "Can't find anything to run".to_string(),
    };

    println!("got output:\n{}", &output);
}

#[test]
fn args() {
    for a in std::env::args() {
        println!("{}", a);
    }
    // panic!();
}
