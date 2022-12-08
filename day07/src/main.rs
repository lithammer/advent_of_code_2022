mod parser {
    use nom::branch::alt;
    use nom::character::complete::{digit1, space1};
    use nom::combinator::{map, map_res, rest, value};
    use nom::sequence::separated_pair;
    use nom::{bytes::complete::tag, IResult};

    use crate::{Command, Output};

    fn parse_command_cd(s: &str) -> IResult<&str, Command> {
        map(
            separated_pair(tag("cd"), space1, rest),
            |(_, path): (&str, &str)| Command::Cd(path),
        )(s)
    }

    fn parse_command_ls(s: &str) -> IResult<&str, Command> {
        value(Command::Ls, tag("ls"))(s)
    }

    fn parse_command(s: &str) -> IResult<&str, Output> {
        map(
            separated_pair(tag("$"), space1, alt((parse_command_cd, parse_command_ls))),
            |(_, c)| Output::Command(c),
        )(s)
    }

    fn parse_stdout_dir(s: &str) -> IResult<&str, Output> {
        map(
            separated_pair(tag("dir"), space1, rest),
            |(_, name): (&str, &str)| Output::Dir { name },
        )(s)
    }

    fn parse_stdout_file(s: &str) -> IResult<&str, Output> {
        let parse_size = map_res(digit1, |s: &str| s.parse::<u64>());
        map(
            separated_pair(parse_size, space1, rest),
            |(size, name): (u64, &str)| Output::File { name, size },
        )(s)
    }

    fn parse_stdout(s: &str) -> IResult<&str, Output> {
        alt((parse_stdout_dir, parse_stdout_file))(s)
    }

    pub fn parse_line(s: &str) -> IResult<&str, Output> {
        alt((parse_command, parse_stdout))(s)
    }
}

#[derive(Clone, Debug)]
pub enum Output<'a> {
    Command(Command<'a>),
    File { name: &'a str, size: u64 },
    Dir { name: &'a str },
}

#[derive(Clone, Debug)]
pub enum Command<'a> {
    Cd(&'a str),
    Ls,
}

fn parse(line: &str) -> Output {
    parser::parse_line(line).map(|(_, o)| o).unwrap()
}

fn disk_usage<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<u64> {
    let mut total = 0;
    let mut subdirs = vec![];

    while let Some(s) = lines.next() {
        match parse(s) {
            Output::Command(c) => match c {
                Command::Cd(name) if name == ".." => break,
                Command::Cd(name) if name == "/" => {}
                Command::Cd(_) => {
                    subdirs.extend(disk_usage(lines));
                    total += subdirs.last().unwrap_or(&0);
                }
                Command::Ls => {}
            },
            Output::File { name: _, size } => total += size,
            Output::Dir { name: _ } => {}
        }
    }

    subdirs.push(total);
    subdirs
}

fn part1(input: &str) -> u64 {
    disk_usage(&mut input.lines())
        .iter()
        .filter(|&s| s < &100_000)
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut sizes = disk_usage(&mut input.lines());
    sizes.sort_unstable();

    let file_system_size = 70_000_000;
    let needed_space = 30_000_000;
    let disk_used = sizes.last().unwrap_or(&0); // Last element contains "/".
    let missing = needed_space - (file_system_size - disk_used);

    sizes
        .into_iter()
        .find(|&s| s >= missing)
        .expect("no single directory big enough was found")
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
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "};

    #[test]
    fn test_part1() {
        assert_eq!(crate::part1(SAMPLE), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(crate::part2(SAMPLE), 24933642);
    }
}
