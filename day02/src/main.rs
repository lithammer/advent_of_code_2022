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
    Rock = 0,
    Paper = 1,
    Scissor = 2,
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
        match ((self as i8) - (other as i8)).rem_euclid(3) {
            0 => Outcome::Draw,
            1 => Outcome::Win,
            2 => Outcome::Loss,
            _ => unreachable!(),
        }
    }

    fn beats(self) -> Self {
        (self as i8 - 1).rem_euclid(3).into()
    }

    fn beaten_by(self) -> Self {
        (self as i8 + 1).rem_euclid(3).into()
    }
}

impl From<i8> for Shape {
    fn from(n: i8) -> Self {
        match n {
            0 => Shape::Rock,
            1 => Shape::Paper,
            2 => Shape::Scissor,
            _ => unreachable!(),
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
                Outcome::Win => a.beaten_by(),
                Outcome::Loss => a.beats(),
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
    fn test_parse_line_shape_shape() {
        let result = SAMPLE
            .lines()
            .map(parse_line::<Shape, Shape>)
            .collect::<Vec<_>>();
        let expect = [
            (Shape::Rock, Shape::Paper),
            (Shape::Paper, Shape::Rock),
            (Shape::Scissor, Shape::Scissor),
        ];
        assert_eq!(result, expect);
    }

    #[test]
    fn test_parse_line_shape_outcome() {
        let result = SAMPLE
            .lines()
            .map(parse_line::<Shape, Outcome>)
            .collect::<Vec<_>>();
        let expect = [
            (Shape::Rock, Outcome::Draw),
            (Shape::Paper, Outcome::Loss),
            (Shape::Scissor, Outcome::Win),
        ];

        assert_eq!(result, expect);
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
