use std::cmp;

use regex::Regex;

advent_of_code::solution!(2);

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn part_one(input: &str) -> Option<u32> {
    let game_id_regex = Regex::new(r"Game (?<id>\d+)").unwrap();
    let color_regex = Regex::new(r"(?<amount>\d+) (?<color>red|green|blue)").unwrap();

    let mut total: u32 = 0;
    for line in input.lines() {
        let mut above_max = false;
        for res in color_regex.captures_iter(line) {
            let color = &res["color"];
            let amount = res["amount"].parse::<u32>().unwrap();

            above_max = match color {
                "red" => amount > MAX_RED,
                "green" => amount > MAX_GREEN,
                "blue" => amount > MAX_BLUE,
                _ => false,
            };

            if above_max {
                break;
            }
        }

        let Some(caps) = game_id_regex.captures(line) else {
            return None;
        };
        let game_id = caps["id"].parse::<u32>().unwrap();

        if above_max {
            continue;
        }

        total += game_id;
    }

    return Some(total);
}

fn max_or_default(current_max: u32, new_value: u32) -> u32 {
    if current_max == 0 {
        new_value
    } else {
        cmp::max(current_max, new_value)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let color_regex = Regex::new(r"(?<amount>\d+) (?<color>red|green|blue)").unwrap();

    let mut total: u32 = 0;
    for line in input.lines() {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for res in color_regex.captures_iter(line) {
            let color = &res["color"];
            let amount = res["amount"].parse::<u32>().unwrap();

            match color {
                "red" => min_red = max_or_default(min_red, amount),
                "green" => min_green = max_or_default(min_green, amount),
                "blue" => min_blue = max_or_default(min_blue, amount),
                _ => {}
            };
        }

        let power = min_red * min_green * min_blue;
        total += power;
    }

    return Some(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
