use num::Integer;
use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(8);

type NodesMap = HashMap<String, (String, String)>;

pub fn part_one(input: &str) -> Option<u64> {
    const START: &str = "AAA";

    let movements_re = Regex::new(r"[RL]+").unwrap();
    let node_re = Regex::new(r"(?<id>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();

    let mut nodes: NodesMap = HashMap::new();

    for node in input.lines().skip(2) {
        let caps = node_re.captures(node).unwrap();
        let id = caps["id"].to_string();
        let left = caps["left"].to_string();
        let right = caps["right"].to_string();

        nodes.insert(id, (left, right));
    }

    let movements = movements_re
        .find(input.lines().nth(0).unwrap())
        .unwrap()
        .as_str();

    let (count, _) = movs_to_first_end(movements, &nodes, START, false);
    return Some(count);
}

fn movs_to_first_end(
    movements: &str,
    nodes: &NodesMap,
    start: &str,
    mut skip_first: bool,
) -> (u64, String) {
    let mut moves = movements.chars().cycle();
    let mut count = 0;

    let mut tile = start;
    while !(tile.ends_with("Z") && !skip_first) {
        skip_first = false;
        let mov = moves.next();
        let (left, right) = nodes.get(tile).unwrap();
        let next = match mov {
            Some('L') => left.as_str(),
            Some('R') => right.as_str(),
            _ => panic!(),
        };

        tile = next;
        count += 1;
    }

    let end = tile;
    return (count, end.to_string());
}

pub fn part_two(input: &str) -> Option<u64> {
    let movements_re = Regex::new(r"[RL]+").unwrap();
    let node_re = Regex::new(r"(?<id>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();

    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    let mut starts: Vec<String> = vec![];

    for node in input.lines().skip(2) {
        let caps = node_re.captures(node).unwrap();

        let id = caps["id"].to_string();
        let left = caps["left"].to_string();
        let right = caps["right"].to_string();

        if id.ends_with("A") {
            starts.push(id.clone());
        }

        graph.insert(id, (left, right));
    }

    let movements = movements_re
        .find(input.lines().nth(0).unwrap())
        .unwrap()
        .as_str();

    let mut visited: HashMap<String, u64> = HashMap::new();
    for start in starts.iter() {
        let mut current = start.to_string();

        while !visited.contains_key(&current) {
            let (count, end) = movs_to_first_end(movements, &graph, &current, true);
            visited.insert(current.to_string(), count);
            current = end.to_string();
        }
    }

    return Some(visited.iter().map(|(_, v)| v).fold(1, |acc, v| v.lcm(&acc)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
