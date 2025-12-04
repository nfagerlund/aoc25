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
        new_day(&day)
            .expect("Something failed when creating a new day. Your workdir is probably mussed.");
        return;
    }

    let puzzle = first_arg;
    let f = dispatch::puzzle_fn(&puzzle);
    let input = input_string(&puzzle);
    let output = f(&input).expect("implementation returned error");

    println!("got output:\n{}", &output);
}

/// Modify our own repository to create a new day's module and input file, and
/// wire everything through main.rs appropriately. Make sure you do this in a
/// clean workdir to avoid losing anything.
fn new_day<'a>(day: &'a str) -> anyhow::Result<()> {
    use std::io::Write;

    let mut days = Vec::<&'a str>::new();
    days.extend_from_slice(dispatch::DAYS);
    days.push(day);

    // New impl module file (copy template)
    std::fs::copy("./day.rs.template", format!("./src/dispatch/day{day}.rs"))?;
    // New input file (empty)
    let inputs = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("./inputs/day{day}.txt"))?;
    drop(inputs);
    // Rewritten main.rs (interpolate template)
    let mut dispatch_file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./src/dispatch.rs")?;
    write!(
        &mut dispatch_file,
        include_str!("../dispatch.rs.template"),
        day_mods = make_day_mods(&days)?,
        puzzle_dispatches = make_puzzle_dispatches(&days)?,
        days_list = make_days_list(&days)
    )?;

    Ok(())
}

fn make_day_mods(days: &[&str]) -> Result<String, std::fmt::Error> {
    use std::fmt::Write;

    let mut out = String::new();
    for &day in days {
        writeln!(&mut out, "pub mod day{day};")?;
    }
    Ok(out)
}

fn make_puzzle_dispatches(days: &[&str]) -> Result<String, std::fmt::Error> {
    use std::fmt::Write;

    let mut out = String::new();
    for &day in days {
        writeln!(&mut out, r#"        "{day}-1" => day{day}::part1,"#)?;
        writeln!(&mut out, r#"        "{day}-2" => day{day}::part2,"#)?;
    }
    Ok(out)
}

fn make_days_list(days: &[&str]) -> String {
    let inner = days.join(r#"", ""#);
    format!(r#""{inner}""#)
}
