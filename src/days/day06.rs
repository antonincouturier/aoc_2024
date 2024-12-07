use rayon::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn run() {
    let (initial_pos, map) =
        read_input("data/day06.txt").expect("Failed to read and parse the input file");

    let result_1 = guard_patrol_count(&initial_pos, &map);
    let result_2 = find_all_loops_parallel(&initial_pos, &map);
    println!("Day 06 - part 1: {}", result_1);
    println!("Day 06 - part 2: {}", result_2);
}

pub struct Map {
    max_i: usize,
    max_j: usize,
    obstacles: HashSet<(usize, usize)>,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Position {
    i: usize,
    j: usize,
    direction: Direction,
}

pub fn read_input(path: &str) -> Result<(Position, Map), Box<dyn Error>> {
    let content = fs::read_to_string(Path::new(path))
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", path));

    let mut obstacles = HashSet::new();
    let mut player: Option<Position> = None;
    let max_i = content.lines().count();
    let max_j = content
        .lines()
        .next()
        .map(|line| line.chars().count())
        .unwrap_or(0);

    for (i, line) in content.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    obstacles.insert((i, j));
                }
                '^' | 'v' | '>' | '<' => {
                    if player.is_some() {
                        return Err("Multiple player positions found".into());
                    }
                    let direction = match ch {
                        '^' => Direction::North,
                        'v' => Direction::South,
                        '>' => Direction::East,
                        '<' => Direction::West,
                        _ => unreachable!(),
                    };
                    player = Some(Position { i, j, direction });
                }
                '.' => {} // No action needed for empty spaces
                other => {
                    return Err(
                        format!("Unrecognized character '{}' at ({}, {})", other, i, j).into(),
                    )
                }
            };
        }
    }
    let player = player.ok_or("No player position found")?;
    let map = Map {
        max_i,
        max_j,
        obstacles,
    };
    Ok((player, map))
}

fn turn_right(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn next_move(position: &Position, map: &Map) -> Option<Position> {
    let (front_i, front_j) = match position.direction {
        Direction::North => {
            if position.i == 0 {
                (usize::MAX, position.j)
            } else {
                (position.i - 1, position.j)
            }
        }
        Direction::South => (position.i + 1, position.j),
        Direction::East => (position.i, position.j + 1),
        Direction::West => {
            if position.j == 0 {
                (position.i, usize::MAX)
            } else {
                (position.i, position.j - 1)
            }
        }
    };

    // Check if out of bonds
    if front_i > map.max_i || front_j > map.max_j {
        return None;
    }

    // Check if obstacle
    if map.obstacles.contains(&(front_i, front_j)) {
        Some(Position {
            i: position.i,
            j: position.j,
            direction: turn_right(&position.direction),
        })
    } else {
        Some(Position {
            i: front_i,
            j: front_j,
            direction: position.direction.clone(),
        })
    }
}

pub fn guard_patrol_count(start: &Position, map: &Map) -> usize {
    let mut current_position = start.clone();
    let mut unique_positions = HashSet::new();

    while let Some(next_pos) = next_move(&current_position, map) {
        unique_positions.insert((current_position.i, current_position.j));
        current_position = next_pos;
    }
    unique_positions.len()
}

fn guard_patrol_loop_found(start: &Position, map: &Map) -> bool {
    let mut current_position = start.clone();
    let mut unique_positions = HashSet::new();
    loop {
        match next_move(&current_position, map) {
            Some(next_pos) => {
                if !unique_positions.insert(current_position.clone()) {
                    return true;
                };
                current_position = next_pos;
            }
            None => {
                return false;
            }
        }
    }
}

pub fn find_all_loops_parallel(start: &Position, map: &Map) -> usize {
    let all_positions: Vec<(usize, usize)> = (0..map.max_i)
        .flat_map(|i| (0..map.max_j).map(move |j| (i, j)))
        .collect();

    all_positions
        .par_iter()
        .filter(|&&(i, j)| !map.obstacles.contains(&(i, j)) || (start.i == i && start.j == j))
        .filter(|&&(i, j)| {
            let mut new_obstacles = map.obstacles.clone();
            new_obstacles.insert((i, j));
            let new_map = Map {
                max_i: map.max_i,
                max_j: map.max_j,
                obstacles: new_obstacles,
            };
            guard_patrol_loop_found(start, &new_map)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard_patrol_count() {
        let obstacles = vec![
            (0, 4),
            (1, 9),
            (3, 2),
            (4, 7),
            (6, 1),
            (7, 8),
            (8, 0),
            (9, 6),
        ]
        .into_iter()
        .collect::<HashSet<(usize, usize)>>();

        let map = Map {
            max_i: 10,
            max_j: 10,
            obstacles,
        };

        let initial_position = Position {
            i: 6,
            j: 4,
            direction: Direction::North,
        };
        let result = guard_patrol_count(&initial_position, &map);
        assert_eq!(
            result, 41,
            "Failed guard patrol count, expected 41 got {}",
            result
        );
    }

    #[test]
    fn test_guard_patrol_loop_found_no_loop() {
        let obstacles = vec![
            (0, 4),
            (1, 9),
            (3, 2),
            (4, 7),
            (6, 1),
            (7, 8),
            (8, 0),
            (9, 6),
        ]
        .into_iter()
        .collect::<HashSet<(usize, usize)>>();

        let map = Map {
            max_i: 10,
            max_j: 10,
            obstacles,
        };

        let initial_position = Position {
            i: 6,
            j: 4,
            direction: Direction::North,
        };

        let has_loop = guard_patrol_loop_found(&initial_position, &map);
        assert!(!has_loop, "Expected no loop, but a loop was detected");
    }

    #[test]
    fn test_guard_patrol_loop_found_with_obstacle() {
        let mut obstacles = vec![
            (0, 4),
            (1, 9),
            (3, 2),
            (4, 7),
            (6, 1),
            (7, 8),
            (8, 0),
            (9, 6),
        ]
        .into_iter()
        .collect::<HashSet<(usize, usize)>>();

        obstacles.insert((6, 3));

        let map = Map {
            max_i: 10,
            max_j: 10,
            obstacles,
        };

        let initial_position = Position {
            i: 6,
            j: 4,
            direction: Direction::North,
        };

        let has_loop = guard_patrol_loop_found(&initial_position, &map);
        assert!(has_loop, "Expected a loop, but no loop was detected");
    }

    #[test]
    fn test_find_all_loops() {
        let initial_obstacles = vec![
            (0, 4),
            (1, 9),
            (3, 2),
            (4, 7),
            (6, 1),
            (7, 8),
            (8, 0),
            (9, 6),
        ]
        .into_iter()
        .collect::<HashSet<(usize, usize)>>();

        let map = Map {
            max_i: 10,
            max_j: 10,
            obstacles: initial_obstacles,
        };

        let initial_position = Position {
            i: 6,
            j: 4,
            direction: Direction::North,
        };

        let loop_count = find_all_loops_parallel(&initial_position, &map);
        assert_eq!(loop_count, 6, "Expected 6 loops, found {}", loop_count);
    }

    /*  This one takes forever but check every so often to find regressions
    #[test]
    fn test_day06_part1_and_part2() {
        let (initial_pos, map) =
            read_input("data/day06.txt").expect("Failed to read test input");
        let result_1 = guard_patrol_count(&initial_pos, &map);
        let result_2 = find_all_loops_parallel(&initial_pos, &map);

        assert_eq!(result_1, 5312, "Day 06 - Part 1 failed"); 
        assert_eq!(result_2, 1748, "Day 06 - Part 2 failed");  
    }
    */
}
