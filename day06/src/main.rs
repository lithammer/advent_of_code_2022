/// Encode lowercase letter as bits in u32.
fn encode_bits(b: &u8) -> u32 {
    1 << (*b as u32 - 'a' as u32)
}

fn is_marker(window: &[u8]) -> bool {
    window
        .iter()
        .map(encode_bits)
        .fold(0, |acc, n| acc | n)
        .count_ones()
        == window.len() as u32
}

fn find_marker(s: &str, size: usize) -> Option<usize> {
    s.as_bytes()
        .windows(size)
        .position(is_marker)
        .map(|p| p + size)
}

fn part1(input: &str) -> usize {
    find_marker(input, 4).unwrap()
}

fn part2(input: &str) -> usize {
    find_marker(input, 14).unwrap()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let samples = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for (input, first_marker) in samples {
            assert_eq!(crate::part1(input), first_marker);
        }
    }

    #[test]
    fn test_part2() {
        let samples = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        for (input, first_marker) in samples {
            assert_eq!(crate::part2(input), first_marker);
        }
    }
}
