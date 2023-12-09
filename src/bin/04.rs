use std::collections::HashSet;

use regex::{NoExpand, Regex};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let preface_re = Regex::new(r"Card\s+\d+:").unwrap();
    let number_re = Regex::new(r"\d+").unwrap();

    let mut points = 0;
    for line in input.lines() {
        let split: Vec<&str> = line.split('|').collect();
        let first_nums = &preface_re.replace(split[0], NoExpand(""));

        let winning: HashSet<u32> = number_re
            .find_iter(first_nums)
            .map(|n| n.as_str().parse::<u32>().unwrap())
            .collect();

        let own: Vec<u32> = number_re
            .find_iter(split[1])
            .map(|n| n.as_str().parse::<u32>().unwrap())
            .collect();

        let mut card_points = 0;
        for number in own {
            if !winning.contains(&number) {
                continue;
            }
            card_points = match card_points {
                0 => 1,
                n => n * 2,
            }
        }

        points += card_points;
    }

    return Some(points);
}

pub fn part_two(input: &str) -> Option<u32> {
    let preface_re = Regex::new(r"Card\s+\d+:").unwrap();
    let number_re = Regex::new(r"\d+").unwrap();

    let mut cards_amount: Vec<u32> = input.lines().map(|_| 1).collect();
    for (card, line) in input.lines().enumerate() {
        let split: Vec<&str> = line.split('|').collect();
        let first_nums = &preface_re.replace(split[0], NoExpand(""));

        let winning: HashSet<u32> = number_re
            .find_iter(first_nums)
            .map(|n| n.as_str().parse::<u32>().unwrap())
            .collect();

        let own: Vec<u32> = number_re
            .find_iter(split[1])
            .map(|n| n.as_str().parse::<u32>().unwrap())
            .collect();

        let matches = own.iter().filter(|n| winning.contains(&n)).count();
        // matches + 1 to be inclusive
        for m in card + 1..card + matches + 1 {
            cards_amount[m] += cards_amount[card];
        }
    }

    return Some(cards_amount.iter().sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
