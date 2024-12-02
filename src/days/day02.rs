use std::fs;
use std::path::Path;

pub fn run() {
    let reports = read_input("data/day02.txt");
    let result_1 = count_safe_reports(&reports);
    let result_2 = count_safe_reports_dampener(&reports);
    println!("Day 02 - part 1: {}", result_1);
    println!("Day 02 - part 2: {}", result_2); 
}

pub fn read_input(path: &str) -> Vec<Vec<i32>> {
    let content = fs::read_to_string(Path::new(path))
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", path));

    let data: Vec<Vec<i32>> = content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect();
    data
}

fn is_difference_safe(difference: &i32) -> bool {
    if *difference == 0 {
        return false; 
    }
    let abs_difference = difference.abs();
    if (abs_difference > 0) && (abs_difference < 4) {
        return true;
    }
    false
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let first = report[0];
    let mut previous_elem = report[1];
    let mut difference = previous_elem - first;

    if ! is_difference_safe(&difference) {
        return false; 
    }
    let is_increasing: bool = difference > 0;

    for &elem in &report[2..] {
        difference = elem - previous_elem;
        if is_increasing != (difference > 0) {
            return false;
        }
        if ! is_difference_safe(&difference) {
            return false; 
        }
        previous_elem = elem;
    }
    true
}

pub fn count_safe_reports(reports: &Vec<Vec<i32>>) -> i32 {
    reports.iter()
        .filter(|report| is_report_safe(report))
        .count()
        .try_into()
        .unwrap()
}

pub fn count_safe_reports_dampener(reports: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for report in reports {
        if is_report_safe(&report) {
            count += 1;
        } else {
            for i in 0..report.len() {
                let report_without_i: Vec<i32> = report
                    .iter()
                    .enumerate()
                    .filter(|&(index, _)| index != i)
                    .map(|(_, &value)| value)
                    .collect();
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
