mod dispatch;
mod meta;

fn input_string(p: &str) -> String {
    let day = p.split('-').next().expect("actually that can't ever fail.");
    let input_file = format!("inputs/day{day}.txt");
    std::fs::read_to_string(input_file).expect("can't find inputs for that")
}

fn main() {
    let mut args = std::env::args();
    args.next(); // burn one (executable name)
    let first_arg = args
        .next()
        .expect("Requires a puzzle argument (like `1-1`) or `make <DAY_NUM>`");

    if first_arg == "make" {
        let day = args
            .next()
            .expect("`make` requires a day number as a second argument");
        meta::new_day(&day)
            .expect("Something failed when creating a new day. Your workdir is probably mussed.");
        return;
    }

    let puzzle = first_arg;
    let f = dispatch::puzzle_fn(&puzzle);
    let input = input_string(&puzzle);
    let output = f(&input).expect("implementation returned error");

    println!("got output:\n{}", &output);
}
