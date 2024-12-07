use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn run() {
    let (first, second) =
        read_input("data/day01.txt").expect("Failed to read and parse the input file");
    let result_1 = sorted_difference(&first, &second);
    let result_2 = similarity_score(&first, &second);
    println!("Day 01 - part 1: {}", result_1);
    println!("Day 01 - part 2: {}", result_2);
}

pub fn read_input(path: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let content = fs::read_to_string(Path::new(path))
        .map_err(|e| format!("Failed to read input file {}: {}", path, e))?;

    let pairs: Vec<(i32, i32)> = content
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            match (parts.next(), parts.next(), parts.next()) {
                (Some(a), Some(b), None) => match (a.parse::<i32>(), b.parse::<i32>()) {
                    (Ok(n1), Ok(n2)) => Some((n1, n2)),
                    _ => None,
                },
                _ => None,
            }
        })
        .collect();

    let list1: Vec<i32> = pairs.iter().map(|(a, _)| *a).collect();
    let list2: Vec<i32> = pairs.iter().map(|(_, b)| *b).collect();

    Ok((list1, list2))
}

pub fn sorted_difference(first: &[i32], second: &[i32]) -> i32 {
    let mut first_sorted = first.to_vec();
    let mut second_sorted = second.to_vec();

    first_sorted.sort();
    second_sorted.sort();

    first_sorted
        .iter()
        .zip(second_sorted.iter())
        .map(|(x, y)| (x - y).abs())
        .sum()
}

pub fn similarity_score(first: &[i32], second: &[i32]) -> i32 {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for &elem in second {
        let count = counter.entry(elem).or_insert(0);
        *count += 1;
    }

    first
        .iter()
        .map(|&x| x * counter.get(&x).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_difference() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        let expected = 11;
        let result = sorted_difference(&list1, &list2);
        assert_eq!(result, expected, "Failed sorted difference")
    }

    #[test]
    fn test_similarity_score() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        let expected = 31;
        let result = similarity_score(&list1, &list2);
        assert_eq!(result, expected, "Failed similarity score")
    }
}
