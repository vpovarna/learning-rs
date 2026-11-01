use std::{collections::HashSet, println};

fn part1(input: &str) -> u32 {
      input.lines().map(calculate_points).sum()
}

fn extract_numbers(line: &str) -> (Vec<i32>, Vec<i32>) {
    let parts: Vec<&str> = line.split(" | ").collect();
    let raw_numbers: Vec<&str> = parts[0].split(": ").collect();

    let wining_numbers: Vec<i32> = raw_numbers[1]
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    let all_numbers: Vec<i32> = parts[1]
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    (wining_numbers, all_numbers)
}

fn calculate_points(line: &str) -> u32 {
    let (winning, have) = extract_numbers(line);
    let matches = have.iter().filter(|n| winning.contains(n)).count() as u32;

    if matches > 0 {
        2_u32.pow(matches - 1)
    } else {
        0
    }
}

fn main() {
    let input =
        std::fs::read_to_string("inputs/2023_day03.txt").expect("Unable to read input file");

    println!("Part1: {}", part1(&input))
}
