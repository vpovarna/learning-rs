use std::{collections::HashMap};

fn part1(input: &str) -> i32 {
    
    let lines: Vec<&str> = input.lines().collect();

    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for line in lines{
        let mut parts = line.split_whitespace();
        v1.push(parts.next().unwrap().parse().unwrap());
        v2.push(parts.next().unwrap().parse().unwrap());
    }
    v1.sort();
    v2.sort();

    return v1.iter()
        .zip(v2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
}


fn part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for line in lines{
        let mut parts = line.split_whitespace();
        v1.push(parts.next().unwrap().parse().unwrap());
        v2.push(parts.next().unwrap().parse().unwrap());
    }

    let mut occurrence: HashMap<i32, i32> = HashMap::new();
    for v in v2 {
        *occurrence.entry(v).or_default() += 1;
    }

    let dist = v1.iter().map(|v| v * occurrence.get(v).unwrap_or(&0)).sum();
    return dist;
}

fn main() {
    let input = std::fs::read_to_string("inputs/2024_day01.txt").expect("missing input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(part1(input), 11)
    }

    #[test]
    fn test_part2() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(part2(input), 31)
    }
}