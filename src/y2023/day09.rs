use rstest::rstest;
use std::time::Instant;

use crate::utils::parser;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", input.lines().map(next_int).sum::<i64>());
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", input.lines().map(back_int).sum::<i64>());
    println!("\t time:{:?}", start_time.elapsed());
}

fn next_int(input: &str) -> i64 {
    let mut sequence = parser::get_number_from_line(input);
    let mut last_numbers: Vec<i64> = vec![];

    while !sequence.iter().all(|num| *num == 0) {
        last_numbers.push(sequence[sequence.len() - 1]);
        sequence = sequence
            .iter()
            .zip(sequence.iter().skip(1))
            .map(|(num1, num2)| num2 - num1)
            .collect();
    }
    last_numbers.iter().sum()
}

fn back_int(input: &str) -> i64 {
    let mut sequence = parser::get_number_from_line(input);
    let mut last_numbers: Vec<i64> = vec![];

    while !sequence.iter().all(|num| *num == 0) {
        last_numbers.push(sequence[0]);
        sequence = sequence
            .iter()
            .zip(sequence.iter().skip(1))
            .map(|(num1, num2)| num1 - num2)
            .collect();
    }
    last_numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(18, "0   3   6   9  12  15")]
    #[case(28, "1 3 6 10 15 21")]
    #[case(68, "10 13 16 21 30 45")]
    #[case(-9, "-1 -3 -5 -7")]
    fn test_individual_next(#[case] expected: i64, #[case] input: &str) {
        assert_eq!(expected, next_int(input))
    }

    #[test]
    fn test_next_int() {
        let inpu = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result: i64 = 114;

        assert_eq!(result, inpu.lines().map(next_int).sum())
    }

    #[rstest]
    #[case(-3, "0   3   6   9  12  15")]
    #[case(0, "1 3 6 10 15 21")]
    #[case(5, "10 13 16 21 30 45")]
    #[case(1, "-1 -3 -5 -7")]
    fn test_individual_back(#[case] expected: i64, #[case] input: &str) {
        assert_eq!(expected, back_int(input))
    }

    #[test]
    fn test_back_int() {
        let inpu = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result: i64 = 2;

        assert_eq!(result, inpu.lines().map(back_int).sum())
    }
}
