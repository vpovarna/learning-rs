use std::collections::HashSet;

fn part1(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();

    let some_value = run(chars, 4);
    match some_value {
        Some(value) => value,
        None => 0,
    }
}

fn part2(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();

    let some_value = run(chars, 14);
    match some_value {
        Some(value) => value,
        None => 0,
    }
}

fn run(chars: Vec<char>, n: usize) -> Option<usize> {
    for i in n-1..(chars.iter().len()) {
        let mut set: HashSet<char> = HashSet::new();
        for j in 0..n {
            set.insert(chars[i - j]);
        }

        if set.iter().len() == n {
            return Some(i + 1);
        }
    }
    None
}

fn main() {
    let input =
        std::fs::read_to_string("inputs/2022_day02.txt").expect("failed to read input file");

    println!("Part1 {}", part1(&input));
    println!("Part2 {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        use std::assert_eq;

        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part2_test() {
        use std::assert_eq;

        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
