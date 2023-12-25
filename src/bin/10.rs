use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

type Pos = (usize, usize);
type PosDirection = (isize, isize);

fn add_to_position(pos: Pos, n: PosDirection) -> Option<Pos> {
    let x: Result<usize, _> = (pos.0 as isize + n.0).try_into();
    let y: Result<usize, _> = (pos.1 as isize + n.1).try_into();

    if x.is_err() || y.is_err() {
        return None;
    } else {
        return Some((x.ok().unwrap(), y.ok().unwrap()));
    }
}

const NORTH: PosDirection = (-1, 0);
const SOUTH: PosDirection = (1, 0);
const WEST: PosDirection = (0, -1);
const EAST: PosDirection = (0, 1);

fn is_inside(grid: &Vec<Vec<char>>, tile: Pos) -> bool {
    // 0 bounds are checked by usize parsing
    return grid.len() > tile.0 && grid[0].len() > tile.1;
}

fn can_receive(direction: PosDirection, symbol: char) -> bool {
    match direction {
        WEST => vec!['S', 'L', 'F', '-'].contains(&symbol),
        EAST => vec!['S', '7', 'J', '-'].contains(&symbol),
        SOUTH => vec!['S', 'L', 'J', '|'].contains(&symbol),
        NORTH => vec!['S', 'F', '7', '|'].contains(&symbol),
        _ => false,
    }
}

fn opposite_direction(direction: PosDirection) -> PosDirection {
    match direction {
        NORTH => SOUTH,
        SOUTH => NORTH,
        WEST => EAST,
        EAST => WEST,
        _ => (0, 0),
    }
}

fn are_connected(grid: &Vec<Vec<char>>, direction: PosDirection, f_pos: Pos, t_pos: Pos) -> bool {
    let from = grid[f_pos.0][f_pos.1];
    let to = grid[t_pos.0][t_pos.1];

    match from {
        '|' => match direction {
            NORTH | SOUTH => can_receive(direction, to),
            _ => false,
        },
        '-' => match direction {
            WEST | EAST => can_receive(direction, to),
            _ => false,
        },
        'L' => match direction {
            NORTH | EAST => can_receive(direction, to),
            _ => false,
        },
        'J' => match direction {
            NORTH | WEST => can_receive(direction, to),
            _ => false,
        },
        '7' => match direction {
            SOUTH | WEST => can_receive(direction, to),
            _ => false,
        },
        'F' => match direction {
            SOUTH | EAST => can_receive(direction, to),
            _ => false,
        },
        '.' => match direction {
            _ => false,
        },
        'S' => can_receive(direction, to),
        _ => false,
    }
}

fn move_tile(
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<Pos>,
    tile: Pos,
    prev_dir: PosDirection,
) -> Option<(Pos, PosDirection)> {
    let moves = vec![NORTH, SOUTH, EAST, WEST];

    for m in moves {
        // avoid going back
        if m == opposite_direction(prev_dir) {
            continue;
        }

        let next = match add_to_position(tile, m) {
            Some(n) => n,
            None => continue,
        };

        if is_inside(grid, next) && !visited.contains(&next) && are_connected(&grid, m, tile, next)
        {
            visited.insert(next);
            return Some((next, m));
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut start: Pos = (0, 0);
    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(x, l)| {
            if let Some(y) = l.find('S') {
                start = (x, y);
            }

            l.chars().collect()
        })
        .collect();

    let mut visited: HashSet<Pos> = HashSet::new();
    let mut current: Pos = start;
    let mut prev_dir: PosDirection = (0, 0);

    (current, prev_dir) = move_tile(&grid, &mut visited, current, prev_dir).unwrap();
    visited.insert(current);

    let mut around_steps = 1;
    while current != start {
        (current, prev_dir) = match move_tile(&grid, &mut visited, current, prev_dir) {
            Some((n, p)) => (n, p),
            None => panic!("no placement for tile"),
        };

        around_steps += 1;
    }

    let steps = around_steps / 2;
    return Some(steps);
}

pub fn part_two(input: &str) -> Option<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }
}
