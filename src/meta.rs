/// Modify our own repository to create a new day's module and input file, and
/// wire everything through main.rs appropriately. Make sure you do this in a
/// clean workdir to avoid losing anything.
pub fn new_day<'a>(day: &'a str) -> anyhow::Result<()> {
    use std::io::Write;

    let mut days = Vec::<&'a str>::new();
    days.extend_from_slice(crate::dispatch::DAYS);
    days.push(day);

    // New impl module file (copy template)
    std::fs::copy("./day.rs.template", format!("./src/dispatch/day{day}.rs"))?;
    // New input file (empty)
    let inputs = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("./inputs/day{day}.txt"))?;
    drop(inputs);
    // Rewritten dispatch.rs (interpolate template)
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

/// `pub mod dayN;`
fn make_day_mods(days: &[&str]) -> Result<String, std::fmt::Error> {
    use std::fmt::Write;

    let mut out = String::new();
    for &day in days {
        writeln!(&mut out, "pub mod day{day};")?;
    }
    Ok(out)
}

/// The inner body of a match statement that produces function items.
fn make_puzzle_dispatches(days: &[&str]) -> Result<String, std::fmt::Error> {
    use std::fmt::Write;

    let mut out = String::new();
    for &day in days {
        writeln!(&mut out, r#"        "{day}-1" => day{day}::part1,"#)?;
        writeln!(&mut out, r#"        "{day}-2" => day{day}::part2,"#)?;
    }
    Ok(out)
}

/// `"1", "2", "3"` to be placed inside a slice literal.
fn make_days_list(days: &[&str]) -> String {
    let inner = days.join(r#"", ""#);
    format!(r#""{inner}""#)
}
