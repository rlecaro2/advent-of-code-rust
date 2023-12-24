use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(9);

fn find_step_recursive(nums: &Vec<i64>) -> i64 {
    let diffs: Vec<i64> = nums.iter().tuple_windows().map(|(a, b)| b - a).collect();
    if diffs.iter().all_equal() {
        return *diffs.last().unwrap();
    } else {
        return *diffs.last().unwrap() + find_step_recursive(&diffs);
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let number_re = Regex::new(r"-?\d+").unwrap();

    let sum = input
        .lines()
        .map(|history| {
            let nums: Vec<i64> = number_re
                .find_iter(history)
                .map(|n| n.as_str().parse::<i64>().unwrap())
                .collect();

            let last = nums.last().unwrap();
            let next = last + find_step_recursive(&nums);
            return next;
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let number_re = Regex::new(r"-?\d+").unwrap();

    let sum = input
        .lines()
        .map(|history| {
            let mut nums: Vec<i64> = number_re
                .find_iter(history)
                .map(|n| n.as_str().parse::<i64>().unwrap())
                .collect();

            nums.reverse();

            let last = nums.last().unwrap();
            let next = last + find_step_recursive(&nums);
            return next;
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
