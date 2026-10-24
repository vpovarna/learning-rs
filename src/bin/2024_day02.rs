use std::println;

fn is_safe(parts: &[i32]) -> bool {
    let is_ascending = parts.windows(2).all(|w| w[1] > w[0] && w[1] - w[0] <= 3);
    let is_descending = parts.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] <= 3);

    return is_ascending || is_descending;
}

fn part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut count = 0;

    for line in lines {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|a| a.parse::<i32>().unwrap())
            .collect();

        if is_safe(&parts) {
            count +=1;
        }
    }

    return count;
}

fn part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut count = 0;

    for line in lines {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|a| a.parse::<i32>().unwrap())
            .collect();

        let safe = is_safe(&parts) || (0..parts.len()).any(|i| {
            let mut reduced = parts.clone();
            reduced.remove(i);
            is_safe(&reduced)
        });

        if safe {
            count +=1;
        }
    }

    return count;
}

fn main() {
    let input = std::fs::read_to_string("inputs/2024_day02.txt").expect("missing file");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
