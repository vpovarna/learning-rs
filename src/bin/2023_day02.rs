use std::println;

#[derive(Debug, Eq, PartialEq, Default)]
struct CubeSet {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    id: i32,
    sets: Vec<CubeSet>,
}

impl Game {
    fn valid_game(&self) -> bool {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in &self.sets {
            max_red = max_red.max(set.red);
            max_blue = max_blue.max(set.blue);
            max_green = max_green.max(set.green);
        }

        max_red <= 12 && max_green <= 13 && max_blue <= 14
    }

    fn fewest_number_of_cubes(&self) -> i32 {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in &self.sets {
            max_red = max_red.max(set.red);
            max_blue = max_blue.max(set.blue);
            max_green = max_green.max(set.green);
        }

        max_red * max_blue * max_green
    }
}

fn parse_line(line: &str) -> Game {
    let parts: Vec<&str> = line.split(":").collect();
    Game {
        id: parse_game(parts[0]),
        sets: parse_sets(parts[1]),
    }
}

fn parse_game(game: &str) -> i32 {
    let parts: Vec<&str> = game.split(" ").collect();
    parts[1].parse::<i32>().unwrap()
}

fn parse_sets(sets: &str) -> Vec<CubeSet> {
    let v: Vec<CubeSet> = sets.split(";").map(|s| parse_set(s)).collect();
    v
}

fn parse_set(set_part: &str) -> CubeSet {
    let parts: Vec<&str> = set_part.split(",").collect();
    let mut cube_set: CubeSet = CubeSet::new();

    for part in parts {
        if part.contains("red") {
            cube_set.red += extract_value(part.trim())
        } else if part.contains("green") {
            cube_set.green += extract_value(part.trim())
        } else if part.contains("blue") {
            cube_set.blue += extract_value(part.trim())
        }
    }

    return cube_set;
}

fn extract_value(set_value: &str) -> i32 {
    let parts: Vec<&str> = set_value.split(" ").collect();
    return parts[0].parse::<i32>().unwrap();
}

fn part1(input_data: &str) -> i32 {
    input_data
        .lines()
        .map(|l| parse_line(l))
        .filter(|g| g.valid_game())
        .map(|g| g.id)
        .sum()
}

fn part2(input_data: &str) -> i32 {
    input_data
        .lines()
        .map(|l| parse_line(l))
        .map(|g| g.fewest_number_of_cubes())
        .sum()
}

fn main() {
    let input =
        std::fs::read_to_string("inputs/2023_day02.txt").expect("Unable to read the input file");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use std::{assert_eq, vec};

    use super::*;

    #[test]
    fn test_extract_value() {
        let test_value = "4 red";

        assert_eq!(4, extract_value(test_value));
    }

    #[test]
    fn test_parse_set() {
        let test_set: &str = " 3 blue, 4 red";
        let test_cube_set = CubeSet {
            red: 4,
            green: 0,
            blue: 3,
        };

        assert_eq!(test_cube_set, parse_set(test_set));
    }

    #[test]
    fn test_parse_sets() {
        let tests_str = " 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected_sets = vec![
            CubeSet {
                red: 4,
                green: 0,
                blue: 3,
            },
            CubeSet {
                red: 1,
                green: 2,
                blue: 6,
            },
            CubeSet {
                red: 0,
                green: 2,
                blue: 0,
            },
        ];

        assert_eq!(expected_sets, parse_sets(tests_str));
    }
}
