use std::{collections::HashSet, iter};

type Point = (i32, i32);

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        }
    }

    fn apply(&self, (x, y): Point) -> Point {
        let (dx, dy) = self.delta();
        (x + dx, y + dy)
    }
}

fn parse(input: &str) -> impl Iterator<Item = Direction> + '_ {
    input.lines().flat_map(|s| {
        let (a, b) = s.split_once(' ').unwrap();
        let steps = b.parse().unwrap();

        let direction = match a {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => unreachable!(),
        };

        iter::repeat(direction).take(steps)
    })
}

fn is_adjacent((x1, y1): Point, (x2, y2): Point) -> bool {
    let (dx, dy) = ((x2 - x1).abs(), (y2 - y1).abs());
    dx <= 1 && dy <= 1
}

fn walk(directions: impl Iterator<Item = Direction>) -> Vec<Point> {
    directions
        .fold((vec![(0, 0)], (0, 0)), |(mut acc, p), d| {
            let p2 = d.apply(p);
            acc.push(p2);
            (acc, p2)
        })
        .0
}

fn follow(head_visits: Vec<Point>, num_tails: u8) -> Vec<Point> {
    let mut tail_visits = vec![(0, 0)];
    let mut tail = (0, 0);

    for head in head_visits {
        if !is_adjacent(head, tail) {
            let ((xhead, yhead), (xtail, ytail)) = (head, tail);

            // Rely on the fact that Ordering has the correct delta values [-1, 0, 1].
            let dx = xhead.cmp(&xtail) as i32;
            let dy = yhead.cmp(&ytail) as i32;

            tail = (xtail + dx, ytail + dy);
            tail_visits.push(tail);
        }
    }

    match num_tails {
        1 => tail_visits,
        _ => follow(tail_visits, num_tails - 1),
    }
}

fn part1(input: &str) -> usize {
    let head = walk(parse(input));
    follow(head, 1).into_iter().collect::<HashSet<_>>().len()
}

fn part2(input: &str) -> usize {
    let head = walk(parse(input));
    follow(head, 9).into_iter().collect::<HashSet<_>>().len()
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
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "};

    const SAMPLE2: &str = indoc! {"
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20
    "};

    #[test]
    fn test_part1() {
        assert_eq!(crate::part1(SAMPLE), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(crate::part2(SAMPLE), 1);
        assert_eq!(crate::part2(SAMPLE2), 36);
    }
}
