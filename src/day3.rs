use anyhow::anyhow;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let mut working_bank = Vec::<u32>::with_capacity(100); // counted the line length.
    let mut sum = 0_u32;
    for line in input.lines() {
        load_bank(&mut working_bank, line);
        let res = process_bank_part1(&working_bank);
        println!("res: {res}");
        sum += res;
    }
    Ok(format!("{}", sum))
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let mut working_bank = Vec::<u32>::with_capacity(100); // counted the line length.
    let mut sum = 0_u32;
    for line in input.lines() {
        load_bank(&mut working_bank, line);
        let res = process_bank_part2(&working_bank, 12);
        println!("res: {res}");
        sum += res;
    }
    Ok(format!("{}", sum))
}

const _EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

#[test]
fn part1_test() {
    assert_eq!(part1(_EXAMPLE).expect("should ok"), "357".to_string());
}

#[test]
fn part_2_test() {
    assert_eq!(
        part2(_EXAMPLE).expect("should ok"),
        "3121910778619".to_string()
    );
}

/// void: replaces the contents of the provided vec with numbers extracted from
/// the line of text.
fn load_bank(dest: &mut Vec<u32>, line: &str) {
    let stuff = line
        .chars()
        .map(|c| c.to_digit(10).expect("malformed input"));
    dest.clear();
    dest.extend(stuff);
}

/// Returns the biggest two-digit number that can be formed from a bank of
/// digits (respecting order, but with any amount of space between them).
fn process_bank_part1(bank: &[u32]) -> u32 {
    process_bank_part2(bank, 2)
}

fn process_bank_part2(bank: &[u32], digits: u32) -> u32 {
    let mut accumulator = 0_u32;
    let mut starting_index = 0_usize;

    // fencepost check: for 2 digits, we want 10^0 and 10^1.
    // work backwards, biggest place first.
    for i in (0..digits).rev() {
        // Locate the biggest number that still leaves space to fill the remaining
        // digits. Note that equal numbers aren't interchangeable; we want the
        // leftmost biggest one, bc it retains the most agency for the remaining digits!

        // len - i: for the tens digit, i would be 1, matching our original impl.
        let max_index = bank.len() - (i as usize);
        let (index, &digit) = bank[starting_index..max_index]
            .iter()
            .enumerate()
            .max_by(|a, b| {
                if a.1 == b.1 {
                    // prefer leftmost, i.e. smallest index
                    b.0.cmp(&a.0)
                } else {
                    a.1.cmp(b.1)
                }
            })
            .expect("bank must have at least n digits");
        // Remember where to start for the next digit
        starting_index = index + 1;
        // put it in place
        accumulator += 10u32.pow(i) * digit;
    }

    accumulator
}

#[test]
fn slicing_test() {
    let s = &[0, 1, 2, 3, 4];
    // well, all right then. good ol off by one.
    assert_eq!(&s[0..(s.len() - 1)], &[0, 1, 2, 3]);
}
