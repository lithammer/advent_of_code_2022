use std::cmp::Ordering;

use parser::Value;

mod parser {
    use std::cmp::Ordering;

    use itertools::EitherOrBoth::{Both, Left, Right};
    use itertools::Itertools;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{map, map_res},
        error::Error,
        multi::separated_list0,
        sequence::delimited,
        Finish, IResult,
    };

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Value {
        Number(u8),
        List(Vec<Value>),
    }

    impl PartialOrd for Value {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Value {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let left = self;
            let right = other;

            match (left, right) {
                (Value::Number(a), Value::Number(b)) => a.cmp(b),
                (Value::Number(a), Value::List(_)) => {
                    let left = &Value::List(vec![Value::Number(*a)]);
                    left.cmp(right)
                }
                (Value::List(_), Value::Number(b)) => {
                    let right = &Value::List(vec![Value::Number(*b)]);
                    left.cmp(right)
                }
                (Value::List(a), Value::List(b)) => {
                    for pair in a.iter().zip_longest(b) {
                        match pair {
                            Both(a, b) => match a.cmp(b) {
                                Ordering::Equal => (), // Keep going.
                                v => return v,
                            },
                            Left(_) => return Ordering::Greater,
                            Right(_) => return Ordering::Less,
                        }
                    }
                    Ordering::Equal
                }
            }
        }
    }

    impl std::str::FromStr for Value {
        type Err = Error<String>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match parse_value(s.as_bytes()).finish() {
                Ok((_remaining, value)) => Ok(value),
                Err(Error { input, code }) => Err(Error {
                    input: String::from_utf8_lossy(input).to_string(),
                    code,
                }),
            }
        }
    }

    impl std::fmt::Display for Value {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Value::Number(n) => n.fmt(f),
                Value::List(vs) => write!(f, "[{}]", vs.iter().map(Value::to_string).join(",")),
            }
        }
    }

    fn parse_number(input: &[u8]) -> IResult<&[u8], u8> {
        map_res(digit1, |b| String::from_utf8_lossy(b).parse())(input)
    }

    fn parse_list(input: &[u8]) -> IResult<&[u8], Vec<Value>> {
        delimited(tag("["), separated_list0(tag(","), parse_value), tag("]"))(input)
    }

    fn parse_value(input: &[u8]) -> IResult<&[u8], Value> {
        alt((
            map(parse_list, Value::List),
            map(parse_number, Value::Number),
        ))(input)
    }

    #[cfg(test)]
    mod tests {
        use std::str::FromStr;

        use super::Value;

        #[test]
        fn test_fromstr() {
            let value: Value = "[[1],4]".parse().unwrap();
            assert_eq!(
                value,
                Value::List(vec![Value::List(vec![Value::Number(1)]), Value::Number(4)])
            );
        }

        #[test]
        fn test_display() {
            let value = Value::from_str("[[1],4]").unwrap();
            assert_eq!(value.to_string(), "[[1],4]",);
        }
    }
}

fn is_ordered(left: &Value, right: &Value) -> bool {
    match left.cmp(right) {
        Ordering::Less | Ordering::Equal => true,
        Ordering::Greater => false,
    }
}

fn part1(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let pairs: Vec<(Value, Value)> = lines
        .split(|line| line.is_empty())
        .map(|v| (v[0].parse().unwrap(), v[1].parse().unwrap()))
        .collect();

    pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| is_ordered(left, right))
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(input: &str) -> usize {
    let mut packets: Vec<Value> = input.lines().filter_map(|s| s.parse().ok()).collect();

    let divider1 = &"[[2]]".parse::<Value>().unwrap();
    packets.push(divider1.clone());
    let divider2 = &"[[6]]".parse::<Value>().unwrap();
    packets.push(divider2.clone());

    packets.sort_unstable();

    packets
        .iter()
        .enumerate()
        .filter(|(_, v)| v == &divider1 || v == &divider2)
        .map(|(i, _)| i + 1)
        .reduce(|acc, i| acc * i)
        .unwrap_or(0)
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
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    "};

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 140);
    }
}
