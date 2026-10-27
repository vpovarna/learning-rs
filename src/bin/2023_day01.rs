use std::println;

fn extract_number(line: &str) -> i32 {
    let characters: Vec<char> = line.chars().collect();

    let digits: Vec<i32> = characters
        .iter()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    return digits.first().unwrap() * 10 + digits.last().unwrap();
}

fn part1(input: &str) -> i32 {
    return input.lines().map(|line| extract_number(line)).sum();
}

fn digit_at(line: &str, characters: &[char], i: usize, words: &[&str]) -> Option<usize> {
    if characters[i].is_ascii_digit() {
        return Some(characters[i].to_digit(10).unwrap() as usize);
    }

    let sub = &line[i..];
    words.iter().position(|w| sub.starts_with(w)).map(|p| p + 1)
}

fn part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut total = 0;

    for line in lines {
        let mut digits: Vec<usize> = Vec::new();
        let characters: Vec<char> = line.chars().collect();

        for i in 0..characters.len() {
            let some_value = digit_at(line, &characters, i, &words);

            match some_value  {
                Some(pos) => digits.push(pos),
                None => continue,
            }
        }
        let value = digits.first().unwrap() * 10 + digits.last().unwrap();
        println!("Line: {}. Value: {}", line, value);
        total += value
    }

    return total as i32;
}

fn main() {
    let input = std::fs::read_to_string("inputs/2023_day01.txt").expect("failed read input file");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
