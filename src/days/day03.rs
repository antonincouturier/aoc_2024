use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn run() {
    let corrupted_memory =
        read_input("data/day03.txt").expect("Failed to read and parse the input file");
    let result_1 = compute_multiplications(&corrupted_memory);
    let result_2 = compute_enabled_multiplications(&corrupted_memory);
    println!("Day 03 - part 1: {}", result_1);
    println!("Day 03 - part 2: {}", result_2);
}

pub fn read_input(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(Path::new(path))
        .map_err(|e| format!("Failed to read input file '{}': {}", path, e))?;

    Ok(content.lines().map(|line| line.to_string()).collect())
}

fn find_valid_mul(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .filter_map(|mul| {
            let x = mul.get(1)?.as_str().parse::<i32>().ok()?;
            let y = mul.get(2)?.as_str().parse::<i32>().ok()?;
            Some((x, y))
        })
        .collect()
}

pub fn compute_multiplications(corrupted_memory: &[String]) -> i32 {
    corrupted_memory
        .iter()
        .flat_map(|input| {
            // Note: need to use flat_map instead of map
            find_valid_mul(input)
                .iter()
                .map(|(x, y)| x * y)
                .collect::<Vec<i32>>()
        })
        .sum()
}

pub fn compute_enabled_multiplications(corrupted_memory: &[String]) -> i32 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
    let mut total = 0;
    let mut enabled = true;

    for line in corrupted_memory {
        for cap in re.captures_iter(line) {
            let matched_str = cap.get(0).unwrap().as_str();
            if matched_str == "do()" {
                enabled = true;
            } else if matched_str == "don't()" {
                enabled = false;
            } else if matched_str.starts_with("mul(") {
                if enabled {
                    let x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    total += x * y;
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_valid_mul() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        let result = find_valid_mul(&input);
        assert_eq!(
            result, expected,
            "Failed find valid mul, expected {:?} got {:?}",
            expected, result
        );
    }

    #[test]
    fn test_compute_multiplication() {
        let input = vec!["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();
        let expected = 161;
        let result = compute_multiplications(&input);
        assert_eq!(
            result, expected,
            "Failed compute multiplications, expected {:?} got {:?}",
            expected, result
        );
    }

    #[test]
    fn test_compute_enabled_multiplications() {
        let input =
            vec!["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>();
        let expected = 48;
        let result = compute_enabled_multiplications(&input);
        assert_eq!(
            result, expected,
            "Failed find do block, expected {:?} got {:?}",
            expected, result
        );
    }
}
