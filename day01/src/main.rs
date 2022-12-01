use std::iter::Iterator;

fn parse_input(input: &str) -> impl Iterator<Item = impl Iterator<Item = u32> + '_> + '_ {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.parse().unwrap()))
}

fn part2(input: &str) -> u32 {
    let mut cals_per_elf = parse_input(input).map(Iterator::sum).collect::<Vec<u32>>();
    cals_per_elf.sort_unstable_by(|a, b| b.cmp(a));
    cals_per_elf.iter().take(3).sum()
}

fn part1(input: &str) -> u32 {
    parse_input(input).map(Iterator::sum).max().unwrap_or(0)
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
