use rayon::prelude::*;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn run() {
    let calibration_data =
        read_input("data/day07.txt").expect("Failed to read and parse the input file");
    let result_1 = total_calibration(&calibration_data);
    let result_2 = total_calibration_concat(&calibration_data);
    println!("Day 07 - part 1: {}", result_1);
    println!("Day 07 - part 2: {}", result_2);
}

pub fn read_input(path: &str) -> Result<Vec<(usize, Vec<usize>)>, Box<dyn Error>> {
    let content = fs::read_to_string(Path::new(path))
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", path));

    let calibration_data = content
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            if parts.len() != 2 {
                return Err(format!("Invalid line {}", line));
            }
            let key: usize = parts[0]
                .parse()
                .map_err(|e| format!("Failed to parse key '{}': {}", parts[0], e))?;
            let values: Vec<usize> = parts[1]
                .split_whitespace()
                .map(|val| {
                    val.parse()
                        .map_err(|e| format!("Failed to parse value '{}': {}", val, e))
                })
                .collect::<Result<Vec<usize>, String>>()?;
            Ok((key, values))
        })
        .collect::<Result<Vec<(usize, Vec<usize>)>, String>>()?
        .into();
    Ok(calibration_data)
}

fn operator_calibration(key: &usize, values: &[usize]) -> bool {
    let mut results: Vec<usize> = vec![values[0]];
    for &value in &values[1..] {
        let mut temp_results: Vec<usize> = Vec::new();
        for result in results {
            temp_results.push(result * value);
            temp_results.push(result + value);
        }
        results = temp_results
            .into_iter()
            .filter(|&value| value <= *key)
            .collect();
    }
    results.contains(key)
}

fn concatenate_integers(i: usize, j: usize) -> usize {
    let concatenated = format!("{}{}", i, j);
    concatenated
        .parse()
        .expect("Failed to parse string to usize")
}

fn operator_calibration_concat(key: &usize, values: &[usize]) -> bool {
    let mut results: Vec<usize> = vec![values[0]];
    for &value in &values[1..] {
        let mut temp_results: Vec<usize> = Vec::new();
        for result in results {
            temp_results.push(result * value);
            temp_results.push(result + value);
            temp_results.push(concatenate_integers(result, value));
        }
        results = temp_results
            .into_iter()
            .filter(|&value| value <= *key)
            .collect();
    }
    results.contains(key)
}

pub fn total_calibration(calibration_data: &[(usize, Vec<usize>)]) -> usize {
    calibration_data
        .par_iter()
        .filter(|(key, values)| operator_calibration(key, values))
        .map(|(key, _)| *key)
        .sum()
}

pub fn total_calibration_concat(calibration_data: &[(usize, Vec<usize>)]) -> usize {
    calibration_data
        .par_iter()
        .filter(|(key, values)| operator_calibration_concat(key, values))
        .map(|(key, _)| *key)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_calibration() {
        let test_cases = vec![
            (190, vec![10, 19], true),
            (3267, vec![81, 40, 27], true),
            (83, vec![17, 5], false),
            (156, vec![15, 6], false),
            (7290, vec![6, 8, 6, 15], false),
            (161011, vec![16, 10, 13], false),
            (192, vec![17, 8, 14], false),
            (21037, vec![9, 7, 18, 13], false),
            (292, vec![11, 6, 16, 20], true),
        ];

        for (key, values, expected) in test_cases {
            let result = operator_calibration(&key, &values);
            assert_eq!(
                result, expected,
                "Failed operator calibration for key: {}, values: {:?}. Expected {}, got {}",
                key, values, expected, result
            );
        }
    }

    #[test]
    fn test_operator_calibration_concat() {
        let test_cases = vec![
            (190, vec![10, 19], true),
            (3267, vec![81, 40, 27], true),
            (83, vec![17, 5], false),
            (156, vec![15, 6], true),
            (7290, vec![6, 8, 6, 15], true),
            (161011, vec![16, 10, 13], false),
            (192, vec![17, 8, 14], true),
            (21037, vec![9, 7, 18, 13], false),
            (292, vec![11, 6, 16, 20], true),
        ];

        for (key, values, expected) in test_cases {
            let result = operator_calibration_concat(&key, &values);
            assert_eq!(
                result, expected,
                "Failed operator calibration for key: {}, values: {:?}. Expected {}, got {}",
                key, values, expected, result
            );
        }
    }
    #[test]
    fn test_total_calibration() {
        let mut input = Vec::new();
        input.push((190, vec![10, 19]));
        input.push((3267, vec![81, 40, 27]));
        input.push((83, vec![17, 5]));
        input.push((156, vec![15, 6]));
        input.push((7290, vec![6, 8, 6, 15]));
        input.push((161011, vec![16, 10, 13]));
        input.push((192, vec![17, 8, 14]));
        input.push((21037, vec![9, 7, 18, 13]));
        input.push((292, vec![11, 6, 16, 20]));

        let expected_sum = 3749;
        let result = total_calibration(&input);
        assert_eq!(
            result, expected_sum,
            "Failed sum_valid_keys, expected sum: {}, got {}",
            expected_sum, result
        );
    }
}
