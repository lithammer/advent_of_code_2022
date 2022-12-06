mod range {
    use std::ops::RangeInclusive;

    use nom::bytes::complete::tag;
    use nom::character::complete::digit1;
    use nom::combinator::{map, map_res};
    use nom::sequence::separated_pair;
    use nom::IResult;

    type Section = u8;
    type Pair = (RangeInclusive<Section>, RangeInclusive<Section>);

    pub fn from_string(s: &str) -> Pair {
        let (_, pair) = parse(s).unwrap();
        pair
    }

    fn parse(s: &str) -> IResult<&str, Pair> {
        map(
            separated_pair(parse_range, tag(","), parse_range),
            |((a0, a1), (b0, b1))| (a0..=a1, b0..=b1),
        )(s)
    }

    fn parse_number(s: &str) -> IResult<&str, u8> {
        map_res(digit1, str::parse)(s)
    }

    fn parse_range(s: &str) -> IResult<&str, (Section, Section)> {
        separated_pair(parse_number, tag("-"), parse_number)(s)
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn test_parse_line() {
            assert_eq!(super::from_string("2-4,6-8"), ((2..=4), (6..=8)));
        }
    }
}

trait RangeInclusiveExt {
    fn contains_range(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl<T> RangeInclusiveExt for std::ops::RangeInclusive<T>
where
    T: PartialOrd,
{
    /// Check if range fully contains other.
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    /// Check if range has any overlap with other.
    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(range::from_string)
        .filter(|(a, b)| a.contains_range(b) || b.contains_range(a))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(range::from_string)
        .filter(|(a, b)| a.overlaps(b) || b.overlaps(a))
        .count()
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
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    fn test_part1() {
        assert_eq!(crate::part1(SAMPLE), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(crate::part2(SAMPLE), 4);
    }
}
