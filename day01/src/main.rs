use itertools::Itertools;

fn parse_line(s: &str) -> u32 {
    s.parse().unwrap()
}

fn parse_input(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.split("\n\n").map(|s| s.lines().map(parse_line).sum())
}

fn part2(input: &str) -> u32 {
    parse_input(input)
        .sorted_unstable_by(|a, b| b.cmp(a)) // Descending.
        .take(3)
        .sum()
}

fn part1(input: &str) -> u32 {
    parse_input(input).max().unwrap_or(0)
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "};

    #[test]
    fn test_part1() {
        assert_eq!(crate::part1(SAMPLE), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(crate::part2(SAMPLE), 45000);
    }
}
