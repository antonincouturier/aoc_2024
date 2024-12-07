use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs;
use std::path::Path;

type Rules = HashMap<u32, Vec<u32>>;
type Pages = Vec<Vec<u32>>;

pub fn run() {
    let (rules, updates) =
        read_input("data/day05.txt").expect("Failed to read and parse the input file");

    let result_1 = middle_page_sum(&updates, &rules);
    let result_2 = reordered_middle_page_sum(&updates, &rules);
    println!("Day 05 - part 1: {}", result_1);
    println!("Day 05 - part 2: {}", result_2);
}

fn extract_rules(input: &str) -> Rules {
    let mut map: Rules = HashMap::new();
    let re = Regex::new(r"^(\d+)\|(\d+)$").unwrap();
    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let key: u32 = caps[1].parse().unwrap();
            let value: u32 = caps[2].parse().unwrap();

            map.entry(key).or_default().push(value);
        }
    }
    map
}

fn extract_pages(input: &str) -> Pages {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|num_str| num_str.trim().parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .collect::<Pages>()
}

pub fn read_input(path: &str) -> Result<(Rules, Pages), Box<dyn Error>> {
    let content = fs::read_to_string(Path::new(path))
        .map_err(|e| format!("Failed to read input file '{}': {}", path, e))?;

    let blocks = content
        .split("\n\n")
        .map(|block| block.trim().to_string())
        .filter(|block| !block.is_empty())
        .collect::<Vec<String>>();

    if blocks.len() != 2 {
        return Err(format!("Expected 2 blocks of input, found {}", blocks.len()).into());
    }
    let rules = extract_rules(&blocks[0]);
    let pages = extract_pages(&blocks[1]);

    Ok((rules, pages))
}

fn check_update(update: &[u32], rules: &Rules) -> bool {
    let mut current_set = HashSet::new();
    for &page in update.iter() {
        if let Some(rule) = rules.get(&page) {
            for &page_rule in rule.iter() {
                if current_set.contains(&page_rule) {
                    return false;
                }
            }
        }
        current_set.insert(page);
    }
    true
}

pub fn middle_page_sum(updates: &[Vec<u32>], rules: &Rules) -> u32 {
    updates
        .iter()
        .filter(|update| check_update(update, rules))
        .map(|update| {
            let len = update.len();
            update[len / 2]
        })
        .sum()
}

fn reorder_update(update: &[u32], rules: &Rules) -> Vec<u32> {
    let unique_pages: HashSet<u32> = update.iter().cloned().collect();
    let mut adjacency_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut degree_count: HashMap<u32, usize> = HashMap::new();

    for &page in unique_pages.iter() {
        if let Some(rule) = rules.get(&page) {
            for &value in rule.iter() {
                if unique_pages.contains(&value) {
                    adjacency_map.entry(page).or_default().push(value);
                    *degree_count.entry(value).or_insert(0) += 1;
                }
            }
        }
    }

    let mut queue: VecDeque<u32> = VecDeque::new();
    let mut page_order: HashMap<u32, usize> = HashMap::new();
    for (idx, &page) in update.iter().enumerate() {
        page_order.insert(page, idx);
    }

    for &page in unique_pages.iter() {
        if !degree_count.contains_key(&page) {
            queue.push_back(page);
        }
    }

    let mut initial_zero_in_degree: Vec<u32> = queue.drain(..).collect();
    initial_zero_in_degree.sort_by_key(|e| page_order[e]);
    queue = initial_zero_in_degree.into();

    let mut sorted = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted.push(node);

        if let Some(neighbors) = adjacency_map.get(&node) {
            for &neighbor in neighbors.iter() {
                if let Some(count) = degree_count.get_mut(&neighbor) {
                    *count -= 1;
                    if *count == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        let mut current_queue: Vec<u32> = queue.drain(..).collect();
        current_queue.sort_by_key(|e| page_order[e]);
        queue = current_queue.into();
    }
    sorted
}

pub fn reordered_middle_page_sum(updates: &[Vec<u32>], rules: &Rules) -> u32 {
    updates
        .iter()
        .filter(|update| !check_update(update, rules))
        .map(|update| {
            let reordered_update = reorder_update(update, rules);
            let len = reordered_update.len();
            reordered_update[len / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_update() {
        let mut rules = HashMap::new();
        rules.insert(47, vec![53, 13, 61, 29]);
        rules.insert(97, vec![13, 61, 47, 29, 53, 75]);
        rules.insert(75, vec![29, 53, 47, 61, 13]);
        rules.insert(61, vec![13, 53, 29]);
        rules.insert(53, vec![29, 13]);
        rules.insert(29, vec![13]);

        let updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let expected_results = vec![true, true, true, false, false, false];
        for (update, expected_result) in updates.iter().zip(expected_results.iter()) {
            let result = check_update(&update, &rules);
            assert_eq!(
                result, *expected_result,
                "Check update failed for {:?}, expected {}, got {} (rules: {:?})",
                update, expected_result, result, rules
            );
        }
    }

    #[test]
    fn test_middle_page_sum() {
        let mut rules = HashMap::new();
        rules.insert(47, vec![53, 13, 61, 29]);
        rules.insert(97, vec![13, 61, 47, 29, 53, 75]);
        rules.insert(75, vec![29, 53, 47, 61, 13]);
        rules.insert(61, vec![13, 53, 29]);
        rules.insert(53, vec![29, 13]);
        rules.insert(29, vec![13]);

        let updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let expected_result = 143;
        let result = middle_page_sum(&updates, &rules);
        assert_eq!(
            result, expected_result,
            "Test middle page sum failes, expected {}, got {}",
            expected_result, result
        );
    }

    #[test]
    fn test_reorder_update() {
        let mut rules = HashMap::new();
        rules.insert(47, vec![53, 13, 61, 29]);
        rules.insert(97, vec![13, 61, 47, 29, 53, 75]);
        rules.insert(75, vec![29, 53, 47, 61, 13]);
        rules.insert(61, vec![13, 53, 29]);
        rules.insert(53, vec![29, 13]);
        rules.insert(29, vec![13]);

        let updates = vec![
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let expected_results = vec![
            vec![97, 75, 47, 61, 53],
            vec![61, 29, 13],
            vec![97, 75, 47, 29, 13],
        ];

        for (update, expected_result) in updates.iter().zip(expected_results.iter()) {
            let result = reorder_update(&update, &rules);
            assert_eq!(
                result, *expected_result,
                "Re-ordering failed for {:?}, expected {:?}, got {:?}",
                update, expected_result, result
            );
        }
    }

    #[test]
    fn test_reordered_middle_page_sum() {
        let mut rules = HashMap::new();
        rules.insert(47, vec![53, 13, 61, 29]);
        rules.insert(97, vec![13, 61, 47, 29, 53, 75]);
        rules.insert(75, vec![29, 53, 47, 61, 13]);
        rules.insert(61, vec![13, 53, 29]);
        rules.insert(53, vec![29, 13]);
        rules.insert(29, vec![13]);

        let updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let expected_result = 123;
        let result = reordered_middle_page_sum(&updates, &rules);
        assert_eq!(
            result, expected_result,
            "Test middle page sum failes, expected {}, got {}",
            expected_result, result
        );
    }

    #[test]
    fn test_day05_part1_and_part2() {
        let (rules, updates) =
            read_input("data/day05.txt").expect("Failed to read test input");
        let result_1 = middle_page_sum(&updates, &rules);
        let result_2 = reordered_middle_page_sum(&updates, &rules);

        assert_eq!(result_1, 5248, "Day 05 - Part 1 failed"); 
        assert_eq!(result_2, 4507, "Day 05 - Part 2 failed"); 
    }
}
