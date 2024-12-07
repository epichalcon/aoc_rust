use itertools::{enumerate, Tuples};
use rstest::rstest;
use std::{char, time::Instant};

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", xmas_occurrances(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}


////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn xmas_occurrances(input: &str) -> u32 {
    let matrix : Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    count_in_rows(&matrix) 
        + count_in_cols(&matrix)
        + count_in_diag(&matrix)
}

fn count_in_rows(matrix: &Vec<Vec<char>>) -> u32 {
    traverse(matrix, (0, 1), "right") 
        + traverse(&reverse_matrix(matrix), (0, 1), "left")
}

fn count_in_cols(matrix: &Vec<Vec<char>>) -> u32 {
    count_in_rows(&transpose_matrix(matrix))
}

fn count_in_diag(matrix: &Vec<Vec<char>>) -> u32 {
    traverse(matrix, (1, 1), "diagonal down right") 
        + traverse(&reverse_matrix(matrix), (1, 1),  "diagonal down left") 
        + traverse(&reverse_matrix(&transpose_matrix(&reverse_matrix(matrix))), (1, 1), "diagonal up left")
        + traverse(&reverse_matrix(&transpose_matrix(&reverse_matrix(&transpose_matrix(&reverse_matrix(matrix))))), (1, 1), "diagonal up right")
}

fn reverse_matrix(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    matrix.into_iter()
        .map(|line| {
            let mut reversed = line.clone();
            reversed.reverse();
            reversed
        })
        .collect()
}

fn transpose_matrix(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let num_cols = matrix[0].len();
    
    // Initialize a new vector of vectors with the transposed shape
    let mut transposed: Vec<Vec<char>> = vec![vec![]; num_cols];

    for row in matrix {
        for (i, val) in row.into_iter().enumerate() {
            transposed[i].push(*val);
        }
    }
    transposed
}


fn traverse(matrix: &Vec<Vec<char>>, dir: (usize, usize), traverse_mode: &str) -> u32{
    let mut count = 0;

    for (i, row) in enumerate(matrix){
        for (j, _) in enumerate(row) {
            if check_word(matrix, i, j, 0, dir) {
                count += 1;
            }
        }
    }

    dbg!(traverse_mode);
    dbg!(count);

    count
}

fn check_word(matrix: &Vec<Vec<char>>, i: usize, j: usize, xmas_i: usize, diff: (usize, usize)) -> bool {
    let xmas = vec!['X', 'M', 'A', 'S'];

    if xmas_i == xmas.len() {
        return true;
    }
    if i == matrix.len() || j == matrix[i].len() {
        return false;
    }
    if matrix[i][j] == xmas[xmas_i]{
        check_word(matrix, i + diff.0, j + diff.1, xmas_i+1, diff)
    }
    else {
        false
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn func2(input: &str) -> u32 {
   1 
}


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(4, "..X...
.SAMX.
.A..A.
XMAS.S
.X....")]
    #[case(18, "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX")]
    fn test_func1(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, xmas_occurrances(input))
    }

    #[rstest]
    #[case(1, "M.S
.A.
M.S")]
    #[case(9, "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX")]
    fn test_func2(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, func2(input))
    }
}
