use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

type TopographicMap = Vec<Vec<usize>>;
type Position = (usize, usize);
type Boundaries = (usize, usize);

pub fn run() {
    let map = read_input("data/day10.txt").expect("Failed to read and parse the input file");
    let result_1 = find_all_paths(&map);
    let result_2 = find_all_ratings(&map);
    println!("Day 10 - part 1: {}", result_1);
    println!("Day 10 - part 2: {}", result_2);
}

pub fn read_input(path: &str) -> Result<TopographicMap, String> {
    let content = fs::read_to_string(Path::new(path))
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", path));

    content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|d| d as usize)
                        .ok_or_else(|| "Failed to parse a digit".to_string())
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<TopographicMap, _>>()
}

fn get_next_positions(
    position: &Position,
    map: &TopographicMap,
    boundaries: &Boundaries,
    height: &usize,
) -> Vec<Position> {
    let mut next_positions: Vec<Position> = Vec::new();
    for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_position = (position.0 as isize + dx, position.1 as isize + dy);
        if new_position.0 < 0 || new_position.1 < 0 {
            continue;
        }
        let new_position = (new_position.0 as usize, new_position.1 as usize);
        if new_position.0 >= boundaries.0 || new_position.1 >= boundaries.1 {
            continue;
        }
        if map[new_position.0][new_position.1] == height + 1 {
            next_positions.push(new_position);
        }
    }
    next_positions
}

fn find_paths(
    starting_position: &Position,
    map: &TopographicMap,
    boundaries: &Boundaries,
) -> usize {
    const MAX_HEIGHT: usize = 9;
    let mut height = map[starting_position.0][starting_position.1];
    if !height == 0 {
        return 0;
    }
    let mut trails: HashSet<Position> = HashSet::new();
    trails.insert(*starting_position);
    while height < MAX_HEIGHT {
        let mut temp_trails = HashSet::new();
        for trail in trails {
            let next_positions = get_next_positions(&trail, map, boundaries, &height);
            for position in next_positions {
                temp_trails.insert(position);
            }
        }
        trails = temp_trails;
        height += 1;
    }
    trails.len()
}

fn find_ratings(
    starting_position: &Position,
    map: &TopographicMap,
    boundaries: &Boundaries,
) -> usize {
    const MAX_HEIGHT: usize = 9;
    let mut height = map[starting_position.0][starting_position.1];
    if !height == 0 {
        return 0;
    }
    let mut trails: Vec<Position> = vec![*starting_position];
    while height < MAX_HEIGHT {
        let mut temp_trails = Vec::new();
        for trail in trails {
            let next_positions = get_next_positions(&trail, map, boundaries, &height);
            for position in next_positions {
                temp_trails.push(position);
            }
        }
        trails = temp_trails;
        height += 1;
    }
    trails.len()
}

pub fn find_all_paths(map: &TopographicMap) -> usize {
    let boundaries = (map.len(), map[0].len());
    let starting_points: Vec<Position> = map
        .iter()
        .enumerate()
        .flat_map(|(index_r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(index_c, &height)| {
                    if height == 0 {
                        Some((index_r, index_c))
                    } else {
                        None
                    }
                })
        })
        .collect();

    let trailheads: Vec<usize> = starting_points
        .par_iter()
        .map(|&position| find_paths(&position, map, &boundaries))
        .collect();

    trailheads.into_iter().sum()
}

pub fn find_all_ratings(map: &TopographicMap) -> usize {
    let boundaries = (map.len(), map[0].len());
    let starting_points: Vec<Position> = map
        .iter()
        .enumerate()
        .flat_map(|(index_r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(index_c, &height)| {
                    if height == 0 {
                        Some((index_r, index_c))
                    } else {
                        None
                    }
                })
        })
        .collect();

    let trailheads: Vec<usize> = starting_points
        .par_iter()
        .map(|&position| find_ratings(&position, map, &boundaries))
        .collect();

    trailheads.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_paths() {
        let map: TopographicMap = vec![
            vec![0, 1, 2, 3],
            vec![1, 2, 3, 4],
            vec![8, 7, 6, 5],
            vec![9, 8, 7, 6],
        ];
        let position = (0, 0);
        let boundaries = (4, 4);
        let result = find_paths(&position, &map, &boundaries);
        assert_eq!(1, result, "Test find paths failed");
    }

    #[test]
    fn test_find_all_paths() {
        let map: TopographicMap = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];
        let result = find_all_paths(&map);
        assert_eq!(36, result, "Test find paths failed");
    }

    #[test]
    fn test_find_all_ratings() {
        let map: TopographicMap = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];
        let result = find_all_ratings(&map);
        assert_eq!(81, result, "Test find paths failed");
    }
    #[test]
    fn test_day10_part1_and_part2() {
        let map = read_input("data/day10.txt").expect("Failed to read and parse the input file");
        let result_1 = find_all_paths(&map);
        let result_2 = find_all_ratings(&map);

        assert_eq!(result_1, 593, "Day 10 - Part 1 failed");
        assert_eq!(result_2, 1192, "Day 09 - Part 2 failed");
    }
}
