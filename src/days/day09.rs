use std::collections::{HashSet, VecDeque};
use std::fs;
use std::path::Path;

pub fn run() {
    let memory = read_input("data/day09.txt").expect("Failed to read and parse the input file");
    let result_1 = process_disk(&memory);
    let result_2 = process_disk_whole_files(&memory);
    println!("Day 09 - part 1: {}", result_1);
    println!("Day 09 - part 2: {}", result_2);
}

pub fn read_input(path: &str) -> Result<Vec<usize>, String> {
    let content = fs::read_to_string(Path::new(path))
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", path));

    content
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|d| d as usize)
                .ok_or_else(|| "Failed to parse a digit".to_string())
        })
        .collect()
}

fn process_disk(memory: &[usize]) -> usize {
    let (block_file, free_space): (Vec<_>, Vec<_>) =
        memory.iter().enumerate().partition(|&(i, _)| i % 2 == 0);

    let mut block_file_fifo: VecDeque<usize> =
        block_file.into_iter().map(|(_, &value)| value).collect();
    let mut free_space_fifo: VecDeque<usize> =
        free_space.into_iter().map(|(_, &value)| value).collect();
    let mut block_file_lifo: Vec<usize> = block_file_fifo.iter().copied().collect();

    let capacity = block_file_fifo.iter().sum();
    let mut rearranged_memory: Vec<usize> = Vec::new();

    let mut current_id = 0;
    let mut block_queue: Vec<usize> = Vec::new();
    let mut last_block = block_file_lifo.pop().expect("Block file lifo empty");
    let mut index = block_file_fifo.len() - 1;
    for _ in 0..last_block {
        block_queue.push(index);
    }

    while let Some(free_space) = free_space_fifo.pop_front() {
        let block_size = block_file_fifo.pop_front().expect("Block file fifo empty");
        for _ in 0..block_size {
            rearranged_memory.push(current_id);
        }
        for _ in 0..free_space {
            if block_queue.is_empty() {
                last_block = block_file_lifo.pop().expect("Block file lifo empty");
                index -= 1;
                for _ in 0..last_block {
                    block_queue.push(index);
                }
            }
            let id_queue = block_queue.pop().expect("Block queue empty");
            rearranged_memory.push(id_queue);
        }
        current_id += 1;
    }

    rearranged_memory = rearranged_memory.into_iter().take(capacity).collect();

    rearranged_memory
        .iter()
        .enumerate()
        .map(|(idx, value)| idx * value)
        .sum()
}

fn process_disk_whole_files(memory: &[usize]) -> usize {
    let (block_file, free_space): (Vec<_>, Vec<_>) =
        memory.iter().enumerate().partition(|&(i, _)| i % 2 == 0);

    let free_space_lifo: Vec<usize> = free_space.into_iter().map(|(_, &value)| value).collect();
    let block_file_lifo: Vec<usize> = block_file.into_iter().map(|(_, &value)| value).collect();

    let mut free_space_lifo: Vec<Vec<Option<usize>>> =
        free_space_lifo.into_iter().map(|n| vec![None; n]).collect();

    let mut rearranged_memory: Vec<Option<usize>> = Vec::new();
    let mut idx_none: HashSet<usize> = HashSet::new();

    for (idx, last_block) in block_file_lifo.iter().enumerate().rev() {
        for (idx_fs, free_space) in free_space_lifo.iter_mut().enumerate() {
            if idx_fs >= idx {
                break;
            }
            let n_free_space = free_space.iter().filter(|x| x.is_none()).count();
            if *last_block <= n_free_space {
                let idx_free_space = free_space.iter().position(|x| x.is_none()).unwrap();
                for idx_insert in idx_free_space..idx_free_space + last_block {
                    free_space[idx_insert] = Some(idx);
                }
                for _ in 0..*last_block {
                    free_space_lifo[idx - 1].push(None);
                }
                idx_none.insert(idx);
                break;
            }
        }
    }

    for ((idx, block), free_space) in block_file_lifo.into_iter().enumerate().zip(free_space_lifo) {
        if !idx_none.contains(&idx) {
            for _ in 0..block {
                rearranged_memory.push(Some(idx));
            }
        }

        rearranged_memory.extend(free_space);
    }

    rearranged_memory
        .iter()
        .enumerate()
        .map(|(idx, value)| value.map_or(0, |v| idx * v))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_disk() {
        let memory: Vec<usize> = vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2];
        let result = process_disk(&memory);
        assert_eq!(1928, result, "Test process disk failed");
    }

    #[test]
    fn test_process_disk_whole_files() {
        let memory: Vec<usize> = vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2];
        let result = process_disk_whole_files(&memory);
        assert_eq!(2858, result, "Test process disk failed");
    }

    #[test]
    fn test_day09_part1_and_part2() {
        let memory = read_input("data/day09.txt").expect("Failed to read and parse the input file");
        let result_1 = process_disk(&memory);
        let result_2 = process_disk_whole_files(&memory);

        assert_eq!(result_1, 6367087064415, "Day 09 - Part 1 failed");
        assert_eq!(result_2, 6390781891880, "Day 09 - Part 2 failed");
    }
}
