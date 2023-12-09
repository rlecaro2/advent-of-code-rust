use itertools::Itertools;
use regex::{Match, Regex};

advent_of_code::solution!(3);

/**
 * Get the sum of values from one line to another.
 * The first one can have symbols. The second numbers.
 * If you want to check within a line, send it in both parameters (since adjacents are diagonals).
*/
fn get_part_numbers(line_with_symbols: &str, line_with_numbers: &str) -> u32 {
    let number_re = Regex::new(r"\d+").unwrap();
    let symbol_re = Regex::new(r"[^\d\.]").unwrap();

    if !symbol_re.is_match(line_with_symbols) || !number_re.is_match(line_with_numbers) {
        return 0;
    }

    let mut sum = 0;
    let numbers: Vec<Match> = number_re.find_iter(line_with_numbers).collect();

    symbol_re
        .find_iter(line_with_symbols)
        .for_each(|symbol_match| {
            //  1  2  3  e
            //  .  #  .
            // s-1 s s+1
            for number_match in numbers.iter() {
                let left = number_match.start() == symbol_match.end();
                let within = number_match.range().contains(&(symbol_match.start()));
                let right = number_match.end() == symbol_match.start();

                if left || within || right {
                    let found = number_match.as_str().parse::<u32>().unwrap();
                    sum += found;
                }

                // Assume numbers are ordered by position
                if number_match.start() > symbol_match.start() {
                    break;
                }
            }
        });

    return sum;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for (prev, next) in input.lines().tuple_windows() {
        // find in this line
        sum += get_part_numbers(prev, prev);
        // find in two directions
        sum += get_part_numbers(prev, next);
        sum += get_part_numbers(next, prev);
    }

    return Some(sum);
}

fn get_gear_ratios(symbol_line: &str, number_lines: Vec<&str>) -> u32 {
    let number_re = Regex::new(r"\d+").unwrap();
    let gear_re = Regex::new(r"\*").unwrap();

    let mut sum = 0;
    for gear in gear_re.find_iter(symbol_line) {
        let numbers = number_lines
            .iter()
            .flat_map(|line| number_re.find_iter(line));

        let mut matches: Vec<u32> = vec![];
        for number_match in numbers {
            let left = number_match.start() == gear.end();
            let within = number_match.range().contains(&(gear.start()));
            let right = number_match.end() == gear.start();

            if left || within || right {
                let found = number_match.as_str().parse::<u32>().unwrap();

                matches.push(found);
            }

            // Assume numbers are ordered by position
            if matches.len() > 2 {
                break;
            }
        }

        if matches.len() == 2 {
            sum += matches[0] * matches[1];
        }
    }

    return sum;
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    for (i, (first, second, third)) in input.lines().tuple_windows().enumerate() {
        if i == 0 {
            sum += get_gear_ratios(first, vec![first, second]);
        }

        sum += get_gear_ratios(second, vec![first, second, third]);

        if i + 1 == input.lines().collect::<Vec<&str>>().len() {
            sum += get_gear_ratios(third, vec![second, third]);
        }
    }

    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
