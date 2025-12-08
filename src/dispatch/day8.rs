use anyhow::anyhow;

/// Connect the *1000* closest-together pairs of boxes to form some number of
/// circuits. Find the sizes of the *three* largest circuits, and multiply them.
pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    part1_real(input, 1000)
}

pub fn part1_real(input: &str, connect: usize) -> anyhow::Result<String> {
    Err(anyhow!("not implemented"))
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    Err(anyhow!("not implemented"))
}

const _EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

// hey notably: this only takes 10 shortest, and the real thing takes 1000. So
// we need diverging impls.
#[test]
fn part1_test() {
    assert_eq!(
        part1_real(_EXAMPLE, 10).expect("should ok"),
        "40".to_string()
    );
}

#[test]
fn part2_test() {
    assert_eq!(part2(_EXAMPLE).expect("should ok"), "LOL".to_string());
}
