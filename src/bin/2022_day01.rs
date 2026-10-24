fn part1(input: &str) -> i32 {
    let calories = calculate_calories(input);
    return calories[0];
}

fn part2(input: &str) -> i32 {
    let calories = calculate_calories(input);
    return calories[0] + calories[1] + calories[2];
}

fn calculate_calories(input: &str) -> Vec<i32> {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut calories: Vec<i32> = Vec::new();

    for part in parts {
        let calory: i32 = part.lines().map(|f| f.parse::<i32>().unwrap()).sum();

        calories.push(calory);
    }

    calories.sort_by(|a, b: &i32| b.cmp(a));
    calories
}

fn main() {
    let input = std::fs::read_to_string("inputs/2022_day01.txt").expect("missing input file");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str =
        "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 24000)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 45000)
    }
}
