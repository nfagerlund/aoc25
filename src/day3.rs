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
    Err(anyhow!("unimplemented"))
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
        "aseothuaestnhu".to_string()
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
    // Locate the biggest number prior to the final element of the bank. Note
    // that equal numbers aren't interchangeable; we want the leftmost biggest
    // one, bc it retains the most agency for the second digit!
    let (index, &first_digit) = bank[0..(bank.len() - 1)]
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
        .expect("bank must have at least two digits");
    let &second_digit = bank[(index + 1)..]
        .iter()
        .max()
        .expect("must be at least one digit left after the first");
    first_digit * 10 + second_digit
}

#[test]
fn slicing_test() {
    let s = &[0, 1, 2, 3, 4];
    // well, all right then. good ol off by one.
    assert_eq!(&s[0..(s.len() - 1)], &[0, 1, 2, 3]);
}
