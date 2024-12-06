use std::fs;
use std::path::Path;
use std::error::Error;

pub fn run() {
    let reports = read_input("data/day02.txt").expect("Failed to read and parse the input file");
    let result_1 = count_safe_reports(&reports);
    let result_2 = count_safe_reports_dampener(&reports);
    println!("Day 02 - part 1: {}", result_1);
    println!("Day 02 - part 2: {}", result_2); 
}

pub fn read_input(path: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let content = fs::read_to_string(Path::new(path))
        .map_err(|e| format!("Failed to read input file '{}': {}", path, e))?;

    Ok(
        content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect()
    )
}

fn is_difference_safe(difference: &i32) -> bool {
    if *difference == 0 {
        return false; 
    }
    const SAFE_THRESHOLD: i32 = 4;
    if difference.abs() < SAFE_THRESHOLD {
        return true;
    }
    false
}

fn is_report_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return false;
    }

    let mut diffs = report.windows(2).map(|pair| pair[1] - pair[0]);
    let first_diff = match diffs.next() {
        Some(d) if is_difference_safe(&d) => d,
        _ => return false,
    };
    let is_increasing = first_diff > 0;
    diffs.all(|diff| is_difference_safe(&diff) && ((diff > 0) == is_increasing))
}

pub fn count_safe_reports(reports: &[Vec<i32>]) -> i32 {
    reports.iter()
        .filter(|report| is_report_safe(report))
        .count()
        .try_into()
        .unwrap()
}

pub fn count_safe_reports_dampener(reports: &[Vec<i32>]) -> i32 {
    let mut count = 0;
    for report in reports {
        if is_report_safe(&report) {
            count += 1;
        } else {
            for i in 0..report.len() {
                let mut report_without_i = report.clone();
                report_without_i.remove(i);
                if is_report_safe(&report_without_i) {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_report_safe() {
        let list1 = vec![7, 6, 4, 2, 1];
        let list2 = vec![1, 2, 7, 8, 9];
        let list3 = vec![9, 7, 6, 2, 1];
        let list4 = vec![1, 3, 2, 4, 5];
        let list5 = vec![8, 6, 4, 4, 1];
        let list6 = vec![1, 3, 6, 7, 9];
        let lists = vec![list1, list2, list3, list4, list5, list6];
        let expected = vec![true, false, false, false, false, true];
        for (list, expected_return) in lists.iter().zip(expected.iter()) {
            let result = is_report_safe(&list);
            assert_eq!(result, *expected_return, "Failed is report safe on report {:?}", list);
        }
    }
    #[test]
    fn test_count_safe_reports() {
        let list1 = vec![7, 6, 4, 2, 1];
        let list2 = vec![1, 2, 7, 8, 9];
        let list3 = vec![9, 7, 6, 2, 1];
        let list4 = vec![1, 3, 2, 4, 5];
        let list5 = vec![8, 6, 4, 4, 1];
        let list6 = vec![1, 3, 6, 7, 9];
        let lists = vec![list1, list2, list3, list4, list5, list6];
        let expected = 2;
        let result = count_safe_reports(&lists);
        assert_eq!(result, expected, "Failed count safe report got {} expected {}", result, expected);
    }
    #[test]
    fn test_count_safe_reports_dampener() {
        let list1 = vec![7, 6, 4, 2, 1];
        let list2 = vec![1, 2, 7, 8, 9];
        let list3 = vec![9, 7, 6, 2, 1];
        let list4 = vec![1, 3, 2, 4, 5];
        let list5 = vec![8, 6, 4, 4, 1];
        let list6 = vec![1, 3, 6, 7, 9];
        let lists = vec![list1, list2, list3, list4, list5, list6];
        let expected = 4;
        let result = count_safe_reports_dampener(&lists);
        assert_eq!(result, expected, "Failed count safe report got {} expected {}", result, expected);
    }
}
