#[derive(Debug)]
enum ParseError {
    UnknownValue(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnknownValue(s) => write!(f, "failed to parse unknown value: {s}"),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Loss => 0,
            Self::Draw => 3,
        }
    }
}

impl std::str::FromStr for Outcome {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            v => Err(ParseError::UnknownValue(v.to_owned())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl Shape {
    fn score(self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }

    fn cmp(self, other: Self) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Scissor)
            | (Self::Scissor, Self::Paper)
            | (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Scissor, Self::Rock)
            | (Self::Paper, Self::Scissor)
            | (Self::Rock, Self::Paper) => Outcome::Loss,
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissor, Self::Scissor) => Outcome::Draw,
        }
    }
}

impl std::str::FromStr for Shape {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissor),
            v => Err(ParseError::UnknownValue(v.to_owned())),
        }
    }
}

fn parse_line<T, U>(s: &str) -> (T, U)
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
    U: std::str::FromStr,
    U::Err: std::fmt::Debug,
{
    let (a, b) = s.split_once(' ').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line::<Shape, Shape>)
        .fold(0, |score, (a, b)| score + b.score() + b.cmp(a).score())
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line::<Shape, Outcome>)
        .fold(0, |score, (a, o)| {
            let b = match o {
                Outcome::Win => match a {
                    Shape::Rock => Shape::Paper,
                    Shape::Paper => Shape::Scissor,
                    Shape::Scissor => Shape::Rock,
                },
                Outcome::Loss => match a {
                    Shape::Rock => Shape::Scissor,
                    Shape::Paper => Shape::Rock,
                    Shape::Scissor => Shape::Paper,
                },
                Outcome::Draw => a,
            };
            score + b.score() + b.cmp(a).score()
        })
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE: &str = "A Y\nB X\nC Z\n";

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("A Z"), (Shape::Rock, Shape::Scissor));
        assert_eq!(parse_line("A Z"), (Shape::Rock, Outcome::Win));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 12);
    }
}
