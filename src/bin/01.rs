use regex::Regex;

advent_of_code::solution!(1);

fn parse_pseudo_digit(pseudo_digit: &str) -> u32 {
    match pseudo_digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        e => e.parse::<u32>().unwrap(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r".*?(?<digit>\d).*?").unwrap();

    let mut total = 0;
    input.lines().for_each(|line| {
        let Some(caps) = re.captures(line) else {
            return;
        };
        let first = caps["digit"].to_owned().clone();

        let reverted: String = line.chars().rev().collect();
        let Some(caps) = re.captures(&reverted) else {
            return;
        };
        let last = caps["digit"].to_owned().clone();

        total += parse_pseudo_digit(&first) * 10;
        total += parse_pseudo_digit(&last);
    });

    return Some(total);
}

pub fn part_two(input: &str) -> Option<u32> {
    let from_start =
        Regex::new(r".*?(?<digit>one|two|three|four|five|six|seven|eight|nine|\d).*?").unwrap();
    // This is just greedy, so we get matched to the last occurrence
    let from_end =
        Regex::new(r".*(?<digit>one|two|three|four|five|six|seven|eight|nine|\d).*?").unwrap();

    let mut total = 0;
    input.lines().for_each(|line| {
        let Some(caps) = from_start.captures(line) else {
            return;
        };
        let first = caps["digit"].to_owned().clone();

        let Some(caps) = from_end.captures(line) else {
            return;
        };
        let last = caps["digit"].to_owned().clone();

        total += parse_pseudo_digit(&first) * 10;
        total += parse_pseudo_digit(&last);
    });

    return Some(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
