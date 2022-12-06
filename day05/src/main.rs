peg::parser! {
    grammar parser() for str {
        rule number() -> usize
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule char() -> char
            = c:['A'..='Z'] { c }

        rule box() -> Option<char>
            = "[" c:(char()) "]" { Some(c) }
            / "   " { None }

        rule layer() -> Vec<Option<char>>
            = v:(box() ++ " ") "\n"? { v }

        pub rule stacks() -> Vec<Vec<char>>
            = v:(layer()*) [_]* { transpose(v) }

        rule procedure() -> Procedure
            = "move " a:(number()) " from " b:(number()) " to " c:(number()) "\n"? {
                Procedure { count: a, from: b-1, to: c-1 }
            }

        pub rule procedures() -> Vec<Procedure>
            = v:(procedure()*) { v }
    }
}

fn transpose<T>(mut v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    for inner in &mut v {
        inner.reverse();
    }
    (0..v[0].len())
        .map(|_| {
            v.iter_mut()
                .filter_map(|inner| inner.pop().unwrap())
                .rev()
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Procedure {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Procedure>) {
    let (stacks, procedures) = input.split_once("\n\n").unwrap();
    (
        parser::stacks(stacks).unwrap(),
        parser::procedures(procedures).unwrap(),
    )
}

fn part1(input: &str) -> String {
    let (mut stacks, procedures) = parse_input(input);

    for p in procedures {
        for _ in 0..p.count {
            let v = stacks[p.from].pop().unwrap();
            stacks[p.to].push(v);
        }
    }

    stacks.iter().map(|v| v.last().unwrap()).collect()
}

fn part2(input: &str) -> String {
    let (mut stacks, procedures) = parse_input(input);

    for p in procedures {
        let n = stacks[p.from].len();
        let mut v = stacks[p.from].split_off(n - p.count);
        stacks[p.to].append(&mut v);
    }

    stacks.iter().map(|v| v.last().unwrap()).collect()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{parse_input, Procedure};

    // Trailing whitespace is intentional.
    const SAMPLE: &str = indoc! {"
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    #[test]
    fn test_parse_stack() {
        let stacks = crate::parser::stacks("[N] [C]     [A]").unwrap();
        assert_eq!(stacks, [vec!['N'], vec!['C'], vec![], vec!['A']]);

        let stacks = crate::parser::stacks("[N]     [A]\n[D] [E] [F]\n 1   2   3 ").unwrap();
        assert_eq!(stacks, [vec!['D', 'N'], vec!['E'], vec!['F', 'A']]);
    }

    #[test]
    fn test_parse_input() {
        let (stacks, procedures) = parse_input(SAMPLE);
        assert_eq!(stacks, [vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P'],]);
        assert_eq!(
            procedures,
            [
                Procedure {
                    count: 1,
                    from: 2 - 1,
                    to: 1 - 1
                },
                Procedure {
                    count: 3,
                    from: 1 - 1,
                    to: 3 - 1
                },
                Procedure {
                    count: 2,
                    from: 2 - 1,
                    to: 1 - 1
                },
                Procedure {
                    count: 1,
                    from: 1 - 1,
                    to: 2 - 1
                },
            ]
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(crate::part1(SAMPLE), "CMZ".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(crate::part2(SAMPLE), "MCD".to_string());
    }
}
