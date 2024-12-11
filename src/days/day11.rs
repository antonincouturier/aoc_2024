use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn run() {
    let line = read_input("data/day11.txt").expect("Failed to read and parse the input file");
    let result_1 = update_line(line.clone(), 25);
    let result_2 = update_line_hashmap(&line, 75);
    println!("Day 11 - part 1: {}", result_1);
    println!("Day 11 - part 2: {}", result_2);
}

pub fn read_input(path: &str) -> Result<Vec<usize>, String> {
    let content = fs::read_to_string(Path::new(path))
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", path));

    content
        .split_whitespace()
        .map(|elem| {
            elem.parse::<usize>()
                .map_err(|e| format!("Failed to parse '{}': {}", elem, e))
        })
        .collect()
}

fn count_digits(number: &usize) -> usize {
    number.to_string().chars().count()
}

fn split_usize(number: &usize, n: &usize) -> Option<(usize, usize)> {
    let number_str = number.to_string();
    let half = n / 2;

    let first: usize = number_str[..half].parse().ok()?;
    let second: usize = number_str[half..].parse().ok()?;

    Some((first, second))
}

fn update_stone(stone: &usize) -> Vec<usize> {
    if *stone == 0 {
        vec![1]
    } else {
        let n = count_digits(stone);
        if n % 2 == 0 {
            match split_usize(stone, &n) {
                Some((stone1, stone2)) => {
                    vec![stone1, stone2]
                }
                None => {
                    Vec::new()
                }
            }
        } else {
            vec![stone * 2024]
        }
    }
}

fn blink(line: &[usize]) -> Vec<usize> {
    line.iter().flat_map(|stone| update_stone(stone)).collect()
}

pub fn update_line(line: Vec<usize>, n_blinks: usize) -> usize {
    let mut result = line;
    for _ in 0..n_blinks {
        result = blink(&result);
    }
    result.len()
}

pub fn update_line_hashmap(line: &[usize], n_blinks: usize) -> usize {
    let mut frequency: HashMap<usize, usize> = HashMap::new();
    for &stone in line {
        *frequency.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..n_blinks {
        let new_counts: Vec<(usize, usize)> = frequency
            .par_iter()
            .flat_map(|(&stone, &count)| {
                let stones = update_stone(&stone);
                stones.into_par_iter().map(move |s| (s, count))
            })
            .collect();

        let mut new_frequency: HashMap<usize, usize> = HashMap::new();
        for (stone, count) in new_counts {
            *new_frequency.entry(stone).or_insert(0) += count;
        }
        frequency = new_frequency;
    }
    frequency.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blink() {
        let line: Vec<usize> = vec![0, 1, 10, 99, 999];
        let expected: Vec<usize> = vec![1, 2024, 1, 0, 9, 9, 2021976];
        let result = blink(&line);
        assert_eq!(expected, result, "Test blink failed");
    }

    #[test]
    fn test_update_line() {
        let line: Vec<usize> = vec![125, 17];
        let n_blinks = 6;
        let expected = 22;
        let result = update_line(line, n_blinks);
        assert_eq!(expected, result, "Test update line failed");
    }

    #[test]
    fn test_update_line_hashmap() {
        let line: Vec<usize> = vec![125, 17];
        let n_blinks = 6;
        let expected = 22;
        let result = update_line_hashmap(&line, n_blinks);
        assert_eq!(expected, result, "Test update line failed");
    }

    #[test]
    fn test_day11_part1_and_part2() {
        let line = read_input("data/day11.txt").expect("Failed to read and parse the input file");
        let result_1 = update_line(line.clone(), 25);
        let result_2 = update_line_hashmap(&line, 75);
        assert_eq!(result_1, 186424, "Day 11 - Part 1 failed");
        assert_eq!(result_2, 219838428124832, "Day 11 - Part 2 failed");
    }
}
