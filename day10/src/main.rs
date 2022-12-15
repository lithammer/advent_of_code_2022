use std::iter;

type Pixels = [[char; 40]; 6];

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn cycles(self) -> i32 {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = std::num::ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.split_once(' ') {
            Some((_, b)) => Ok(Instruction::Addx(b.parse()?)),
            None => Ok(Instruction::Noop),
        }
    }
}

fn eval(input: &str) -> impl Iterator<Item = i32> + '_ {
    let mut input = input
        .lines()
        .map(|s| Instruction::try_from(s).unwrap())
        .map(|i| (i, i.cycles()));
    let mut current = input.next();
    let mut x = 1;

    iter::from_fn(move || {
        current.as_ref()?;

        let (ins, cycles_left) = current.as_mut().unwrap();
        *cycles_left -= 1;
        let current_x = x;
        if *cycles_left == 0 {
            match ins {
                Instruction::Noop => {}
                Instruction::Addx(dx) => x += *dx,
            };
            current = input.next();
        }

        Some(current_x)
    })
}

fn draw(pixels: &Pixels) -> String {
    let mut output = String::with_capacity(6 * 40 + 6);
    for row in pixels {
        output.extend(row);
        output.push('\n');
    }
    output
}

fn is_signal_strength_cycle(cycle: i32) -> bool {
    cycle == 20 || cycle % 40 == 20
}

fn part1(input: &str) -> i32 {
    eval(input)
        .enumerate()
        .map(|(i, x)| (i as i32 + 1, x))
        .fold(0, |total_signal_strength, (cycle, x)| {
            if is_signal_strength_cycle(cycle) {
                total_signal_strength + cycle * x
            } else {
                total_signal_strength
            }
        })
}

fn part2(input: &str) -> String {
    let mut pixels: Pixels = [['.'; 40]; 6];
    for (i, x) in eval(input).enumerate() {
        let sprite = x as isize;
        let px = i % 40;
        let py = i / 40;
        if sprite - 1 == px as isize || sprite == px as isize || sprite + 1 == px as isize {
            pixels[py][px] = '#';
        }
    }

    draw(&pixels)
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1 = {}", part1(input));
    println!("Part 2 = \n{}", part2(input,));
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    "};

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 13140);
    }

    #[test]
    fn test_part2() {
        let output = indoc! {"
            ##..##..##..##..##..##..##..##..##..##..
            ###...###...###...###...###...###...###.
            ####....####....####....####....####....
            #####.....#####.....#####.....#####.....
            ######......######......######......####
            #######.......#######.......#######.....
        "};
        assert_eq!(super::part2(SAMPLE), output);
    }
}
