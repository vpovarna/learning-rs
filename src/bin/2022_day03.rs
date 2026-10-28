use std::{collections::HashSet, panic, println};


fn get_common_item(rucksack: &str) -> char {
    let m = rucksack.len() / 2;

    let first_half = &rucksack[0..m];
    let second_half = &rucksack[m..];

    let unique_first_half_items: HashSet<char> = first_half.chars().collect();
    let unique_second_half_items: HashSet<char> = second_half.chars().collect();

    let common_items: HashSet<char> = unique_first_half_items.intersection(&unique_second_half_items).copied().collect();

    *common_items.iter().next().unwrap()
}


// In rust 'a' = 97 and 'A' = 65
fn priority(c: char) -> i32 {
    if c.is_lowercase() {
        (c as u8 - b'a' + 1) as i32
    } else {
        (c as u8 - b'A' + 27) as i32
    }
}

fn part1(input: &str) -> i32 {
    input.lines()
    .map(|line| get_common_item(line) )
    .map(priority)
    .sum()
}


fn part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut total = 0;
    for group in lines.chunks(3) {
        let elf1 = group[0];
        let elf2 = group[1];
        let elf3 = group[2];

        let elf1_items: HashSet<char> = elf1.chars().collect();
        let elf2_items: HashSet<char> = elf2.chars().collect();
        let elf3_items: HashSet<char> = elf3.chars().collect();

        let ab: HashSet<char> = elf1_items.intersection(&elf2_items).copied().collect();
        let common: HashSet<char>= ab.intersection(&elf3_items).copied().collect();

        let common_char: char = *common.iter().next().unwrap();

        total += priority(common_char)
    }

    total
}

fn main() {
    let some_input = std::fs::read_to_string("inputs/2022_day03.txt");

    let input = match some_input  {
        Ok(input) => input,
        Err(err) => panic!("failed to read input. Error: {}", err),
    };

    println!("Part1 {}", part1(&input));
    println!("Part2 {}", part2(&input));
}

#[cfg(test)]
mod test {
    use std::assert_eq;

use super::*;

    #[test]
    fn test_rage() {
        let cases = [
            ("vJrwpWtwJgWrhcsFMMfFFhFp", 'p'),
            ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 'L'),
            ("PmmdzqPrVvPwwTWBwg", 'P')
        ];
        
        for (input, expected) in cases {
            assert_eq!(get_common_item(input), expected);
        }
        
    }
}