use itertools::Itertools;

type Bits = u64;

fn bits(line: &str) -> Bits {
    line.chars()
        .map(|c| match c {
            'a'..='z' => c as u32 - 'a' as u32,
            'A'..='Z' => c as u32 - 'A' as u32 + 26,
            _ => unreachable!(),
        })
        .fold(0, |bits, bit| bits | 1 << bit)
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let common = bits(a) & bits(b);
            Bits::BITS - common.leading_zeros()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            let common = bits(a) & bits(b) & bits(c);
            Bits::BITS - common.leading_zeros()
        })
        .sum()
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
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    #[test]
    fn test_part1() {
        assert_eq!(crate::part1(SAMPLE), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(crate::part2(SAMPLE), 70);
    }
}
