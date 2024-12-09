use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

type Position = (usize, usize);
type FrequencyMap = HashMap<char, Vec<Position>>;
type Boundaries = (usize, usize);

pub fn run() {
    let (frequency_map, map_boundaries) =
        read_input("data/day08.txt").expect("Failed to read and parse the input file");
    let result_1 = find_all_antinodes(&frequency_map, &map_boundaries);
    let result_2 = find_all_antinodes_resonant(&frequency_map, &map_boundaries);
    println!("Day 08 - part 1: {}", result_1);
    println!("Day 08 - part 2: {}", result_2);
}

pub fn read_input(path: &str) -> Result<(FrequencyMap, Boundaries), String> {
    let content = fs::read_to_string(Path::new(path))
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", path));

    let max_row = content.lines().count();
    let max_col = content
        .lines()
        .next()
        .map_or(0, |line| line.chars().count());

    let frequency_map = content
        .lines()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            line.chars().enumerate().filter_map(move |(col_idx, ch)| {
                if ch.is_alphanumeric() {
                    Some((ch, (row_idx, col_idx)))
                } else {
                    None
                }
            })
        })
        .fold(HashMap::new(), |mut map, (ch, pos)| {
            map.entry(ch).or_insert_with(Vec::new).push(pos);
            map
        });

    Ok((frequency_map, (max_row, max_col)))
}

fn is_in_boundaries(position: &Position, boundaries: &Boundaries) -> bool {
    position.0 < boundaries.0 && position.1 < boundaries.1
}

fn find_antinodes(
    position_1: &Position,
    position_2: &Position,
    boundaries: &Boundaries,
) -> HashSet<Position> {
    let (x1, y1) = (position_1.0 as isize, position_1.1 as isize);
    let (x2, y2) = (position_2.0 as isize, position_2.1 as isize);

    let dx = x2 - x1;
    let dy = y2 - y1;

    let antinode_right = ((x2 + dx) as usize, (y2 + dy) as usize);
    let antinode_left = ((x1 - dx) as usize, (y1 - dy) as usize);

    let mut antinodes = HashSet::new();
    if is_in_boundaries(&antinode_right, boundaries) {
        antinodes.insert(antinode_right);
    }
    if is_in_boundaries(&antinode_left, boundaries) {
        antinodes.insert(antinode_left);
    }
    antinodes
}

fn generate_combinations(positions: &[Position]) -> Vec<(&Position, &Position)> {
    let mut pairs = Vec::new();
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            pairs.push((&positions[i], &positions[j]));
        }
    }
    pairs
}

pub fn find_all_antinodes(frequency_map: &FrequencyMap, boundaries: &Boundaries) -> usize {
    frequency_map
        .par_iter()
        .flat_map(|(_key, positions)| {
            let pairs = generate_combinations(positions);
            pairs
                .par_iter()
                .flat_map(|(p1, p2)| find_antinodes(p1, p2, boundaries))
                .collect::<HashSet<Position>>()
        })
        .collect::<HashSet<Position>>()
        .len()
}

fn find_antinodes_resonant(
    position_1: &Position,
    position_2: &Position,
    boundaries: &Boundaries,
) -> HashSet<Position> {
    let (x1, y1) = (position_1.0 as isize, position_1.1 as isize);
    let (x2, y2) = (position_2.0 as isize, position_2.1 as isize);

    let dx = x2 - x1;
    let dy = y2 - y1;

    let mut antinodes = HashSet::new();
    antinodes.insert(*position_1);
    antinodes.insert(*position_2);

    let mut curr_x = x2 + dx;
    let mut curr_y = y2 + dy;
    while is_in_boundaries(&(curr_x as usize, curr_y as usize), boundaries) {
        antinodes.insert((curr_x as usize, curr_y as usize));
        curr_x += dx;
        curr_y += dy;
    }

    let mut curr_x = x1 - dx;
    let mut curr_y = y1 - dy;
    while is_in_boundaries(&(curr_x as usize, curr_y as usize), boundaries) {
        antinodes.insert((curr_x as usize, curr_y as usize));
        curr_x -= dx;
        curr_y -= dy;
    }
    antinodes
}

pub fn find_all_antinodes_resonant(frequency_map: &FrequencyMap, boundaries: &Boundaries) -> usize {
    frequency_map
        .par_iter()
        .flat_map(|(_key, positions)| {
            let pairs = generate_combinations(positions);
            pairs
                .par_iter()
                .flat_map(|(p1, p2)| find_antinodes_resonant(p1, p2, boundaries))
                .collect::<HashSet<Position>>()
        })
        .collect::<HashSet<Position>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_antinodes() {
        let position_1: Position = (3, 4);
        let position_2: Position = (5, 5);
        let boundaries: Boundaries = (10, 10);

        let result = find_antinodes(&position_1, &position_2, &boundaries);
        let expected = HashSet::from([(1, 3), (7, 6)]);
        assert_eq!(
            result, expected,
            "Antinode positions do not match the expected output."
        );
    }

    #[test]
    fn test_find_all_antinodes() {
        let mut frequency_map: FrequencyMap = HashMap::new();

        frequency_map.insert('0', vec![(1, 8), (2, 5), (3, 7), (4, 4)]);

        frequency_map.insert('A', vec![(5, 6), (8, 8), (9, 9)]);

        let boundaries: Boundaries = (12, 12);

        let result = find_all_antinodes(&frequency_map, &boundaries);
        assert_eq!(
            result, 14,
            "The total antinode count did not match the expected value."
        );
    }

    #[test]
    fn test_find_antinodes_resonant() {
        let boundaries: Boundaries = (10, 10);

        let expected_1: HashSet<Position> =
            [(0, 0), (1, 3), (2, 6), (3, 9)].iter().cloned().collect();
        let result_1 = find_antinodes_resonant(&(0, 0), &(1, 3), &boundaries);
        assert_eq!(
            result_1, expected_1,
            "Combination (0,0)&(1,3) antinodes do not match expected."
        );

        let expected_2: HashSet<Position> = [(0, 0), (2, 1), (4, 2), (6, 3), (8, 4)]
            .iter()
            .cloned()
            .collect();
        let result_2 = find_antinodes_resonant(&(0, 0), &(2, 1), &boundaries);
        assert_eq!(
            result_2, expected_2,
            "Combination (0,0)&(2,1) antinodes do not match expected."
        );

        let expected_3: HashSet<Position> = [(1, 3), (2, 1), (0, 5)].iter().cloned().collect();
        let result_3 = find_antinodes_resonant(&(1, 3), &(2, 1), &boundaries);
        assert_eq!(
            result_3, expected_3,
            "Combination (1,3)&(2,1) antinodes do not match expected."
        );
    }

    #[test]
    fn test_find_all_antinodes_resonant() {
        let mut frequency_map: FrequencyMap = HashMap::new();

        frequency_map.insert('0', vec![(1, 8), (2, 5), (3, 7), (4, 4)]);

        frequency_map.insert('A', vec![(5, 6), (8, 8), (9, 9)]);

        let boundaries: Boundaries = (12, 12);

        let result = find_all_antinodes_resonant(&frequency_map, &boundaries);
        assert_eq!(
            result, 34,
            "The total antinode count did not match the expected value."
        );
    }

    #[test]
    fn test_day08_part1_and_part2() {
        let (frequency_map, map_boundaries) =
            read_input("data/day08.txt").expect("Failed to read and parse the input file");
        let result_1 = find_all_antinodes(&frequency_map, &map_boundaries);
        let result_2 = find_all_antinodes_resonant(&frequency_map, &map_boundaries);

        assert_eq!(result_1, 364, "Day 08 - Part 1 failed");
        assert_eq!(result_2, 1231, "Day 08 - Part 2 failed");
    }
}
