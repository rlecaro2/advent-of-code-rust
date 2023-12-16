use regex::Regex;

advent_of_code::solution!(6);

fn quadratic_zero(a: f32, b: f32, c: f32) -> (f32, f32) {
    (
        (-b + (b.powf(2.0) - 4.0 * a * c).sqrt()) / (2.0 * a),
        (-b - (b.powf(2.0) - 4.0 * a * c).sqrt()) / (2.0 * a),
    )
}

// Paired with @asparagus
pub fn part_one(input: &str) -> Option<u64> {
    let number_re = Regex::new(r"\d+").unwrap();

    let time_line = input.lines().nth(0).unwrap();
    let times: Vec<u64> = number_re
        .find_iter(time_line)
        .map(|d| d.as_str().parse::<u64>().unwrap())
        .collect();

    let distance_line = input.lines().nth(1).unwrap();
    let distances: Vec<u64> = number_re
        .find_iter(distance_line)
        .map(|d| d.as_str().parse::<u64>().unwrap())
        .collect();

    let result = times
        .iter()
        .zip(distances)
        .map(|(&time, distance)| (time as f32, distance as f32))
        .map(|(time, distance)| {
            // distance = charge*time - charge^2
            // -charge^2 + charge*time - distance > 0

            let (left, right) = quadratic_zero(-1.0, time, -distance);
            // +1 to consider one of the two
            let mut res = 1 + (right.floor() - left.ceil()) as u64;

            if left == left.ceil() {
                res -= 1;
            }
            if right == right.floor() {
                res -= 1;
            }

            return res;
        })
        .fold(1, |acc, e| acc * e);

    return Some(result);
}

pub fn part_two(input: &str) -> Option<u64> {
    let whitespace_re = Regex::new(r"[^\S\r\n]").unwrap();
    let clean_input = whitespace_re.replace_all(input, "");

    return part_one(&clean_input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
