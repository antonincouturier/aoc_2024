use std::fs;
use std::path::Path;
use regex::Regex;
use std::cmp::{min, max};
use std::error::Error;

pub fn run() {
    let puzzle = read_input("data/day04.txt").expect("Failed to read and parse the input file");
    let result_1 = count_all_xmas(&puzzle);
    let result_2 = count_all_x_mas(&puzzle);
    println!("Day 04 - part 1: {}", result_1);
    println!("Day 04 - part 2: {}", result_2); 
}

pub fn read_input(path: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let content = fs::read_to_string(Path::new(path))
        .map_err(|e| format!("Failed to read input file '{}': {}", path, e))?;

    Ok(
        content
        .lines()
        .map(|line| {
            line
                .chars()
                .collect::<Vec<char>>()
            
        })
        .collect()
    )
}

fn count_xmas_samx(input: &str) -> usize {
    let re = Regex::new(r"XMAS").unwrap();
    let reverse_input: String = input.chars().rev().collect();
    re.find_iter(input).count() + re.find_iter(&reverse_input).count()
}

pub fn count_all_xmas(puzzle: &Vec<Vec<char>>) -> usize {
    let n_rows = puzzle.len();
    let n_cols = puzzle[0].len();

    // Horizontal 
    let horizontal_count: usize = puzzle
        .iter()
        .map(|row| {
            let row_str = row.iter().collect::<String>();
            count_xmas_samx(&row_str)
        })
        .sum();

    // Vertical 
    let vertical_count: usize = (0..n_cols)
        .map(|col| {
            let col_str = (0..n_rows).map(|row| puzzle[row][col]).collect::<String>();
            count_xmas_samx(&col_str)
        })
        .sum();

    // Diagonals top left 
    let diagonal_tl_count: usize = (3..(n_rows + n_cols - 1)) // at least 4 characters
        .filter_map(|start| {
            let mut diagonal = String::new();
            for i in 0..=start {
                let x = i;
                let y = start - i;
                if x < n_rows && y < n_cols {
                    diagonal.push(puzzle[x][y]);
                }
            }
            if diagonal.len() >= 4 {
                Some(count_xmas_samx(&diagonal))
            } else {
                None
            }
        })
        .sum();

    // Diagonals top right
    let diagonal_tr_count: usize = (3..(n_rows + n_cols - 1)) // at least 4 characters
        .filter_map(|start| {
            let mut diagonal = String::new();
            for i in max(0, start as isize - (n_cols as isize - 1)) as usize..=min(start, n_rows - 1) {
                let x = i;
                let y = n_cols - 1 - (start - i);
                if x < n_rows && y < n_cols {
                    diagonal.push(puzzle[x][y]);
                }
            }
            if diagonal.len() >= 4 {
                Some(count_xmas_samx(&diagonal))
            } else {
                None
            }
        })
        .sum();

    horizontal_count + vertical_count + diagonal_tr_count + diagonal_tl_count
}

pub fn count_all_x_mas(puzzle: &Vec<Vec<char>>) -> usize {
    let n_rows = puzzle.len();
    let n_cols = puzzle[0].len();
    let mut count = 0;
    for i in 1..n_rows - 1 { 
        for j in 1..n_cols - 1 {
            // Only consider case where we find an A 
            if puzzle[i][j] != 'A' {
                continue;
            }

            // Build the diagonals
            let diag_tl = vec![puzzle[i - 1][j - 1], puzzle[i][j], puzzle[i + 1][j + 1]];
            let diag_tr = vec![puzzle[i - 1][j + 1], puzzle[i][j], puzzle[i + 1][j - 1]];
            let diag_tl_str = diag_tl.iter().collect::<String>();
            let diag_tr_str = diag_tr.iter().collect::<String>();
            if (diag_tl_str == "MAS" || diag_tl_str == "SAM") && (diag_tr_str == "MAS" || diag_tr_str == "SAM") {
                count += 1;
            }
        }
    }
    count
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_xmas_samx() {
        let input = "XMASAMX.MM";
        let expected = 2;
        let result = count_xmas_samx(&input);
        assert_eq!(result, expected, "Failed count xmas samx, expected {:?} got {:?}", expected, result);
    }

    #[test]
    fn test_count_all_xmas() {
        let input = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];
        let expected = 18;
        let result = count_all_xmas(&input);
        assert_eq!(result, expected, "Failed count all xmas, expected {:?} got {:?}", expected, result);
    }

    #[test]
    fn test_count_all_x_mas() {
        let input = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];
        let expected = 9;
        let result = count_all_x_mas(&input);
        assert_eq!(result, expected, "Failed count all xmas, expected {:?} got {:?}", expected, result);
    }
}
