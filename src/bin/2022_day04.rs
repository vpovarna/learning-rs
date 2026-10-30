use core::panic;
use std::{println, vec};

fn part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut total = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(",").collect();
        let interval1 = parts[0];
        let interval2 = parts[1];

        let interval1_parts: Vec<i32> = interval1
            .split("-")
            .map(|p| p.parse::<i32>().unwrap())
            .collect();
        let interval2_parts: Vec<i32> = interval2
            .split("-")
            .map(|p| p.parse::<i32>().unwrap())
            .collect();

        if (interval1_parts[0] <= interval2_parts[0] && interval2_parts[1] <= interval1_parts[1])
            || (interval2_parts[0] <= interval1_parts[0]
                && interval1_parts[1] <= interval2_parts[1])
        {
            total += 1;
        }
    }
    return total;
}

fn part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut total = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(",").collect();
        let interval1 = parts[0];
        let interval2 = parts[1];

        let interval1_parts: Vec<i32> = interval1
            .split("-")
            .map(|p| p.parse::<i32>().unwrap())
            .collect();
        let interval2_parts: Vec<i32> = interval2
            .split("-")
            .map(|p| p.parse::<i32>().unwrap())
            .collect();

        if interval1_parts[0] <= interval2_parts[1] && interval2_parts[0] <= interval1_parts[1] {
            total += 1;
        }
    }
    return total;
}

fn main() {
    let some_input = std::fs::read_to_string("inputs/2022_day04.txt");

    let input = match some_input {
        Ok(input) => input,
        Err(_) => panic!("Unable to read input file"),
    };

    println!("Part1 {}", part1(&input));
    println!("Part2 {}", part2(&input));

}
