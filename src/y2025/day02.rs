use std::time::Instant;

use itertools::Itertools;
use nom::{combinator::value, number};
use num::{Integer, PrimInt};

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn construct_duplicated_digit(n: u64) -> u64 {
    let n_digits = n.checked_ilog10().unwrap_or(0) + 1;
    n * 10.pow(n_digits) + n
}

fn func1(input: &str) -> u64 {
    input
        .strip_suffix("\n")
        .unwrap()
        .split(",")
        .fold(0, |mut acc, ran| {
            let values: Vec<u64> = ran.split("-").map(|x| x.parse::<u64>().unwrap()).collect();

            let digits0 = values[0].checked_ilog10().unwrap_or(0) + 1;

            let mut potential_halve_digit = if digits0 % 2 == 1 {
                10.pow(digits0 / 2)
            } else {
                values[0] / 10.pow(digits0 / 2)
            };

            while construct_duplicated_digit(potential_halve_digit) < values[0] {
                potential_halve_digit += 1;
            }

            while construct_duplicated_digit(potential_halve_digit) <= values[1] {
                acc += construct_duplicated_digit(potential_halve_digit);
                potential_halve_digit += 1;
            }

            acc
        })
}

fn construct_repeated_digit(n: u64, repetitions: u32) -> u64 {
    let mut r = 1;
    let mut res = n;
    while r < repetitions {
        let n_digits = n.checked_ilog10().unwrap_or(0) + 1;
        res = res * 10.pow(n_digits) + n;
        r += 1;
    }
    res
}

fn func2(input: &str) -> u64 {
    input
        .strip_suffix("\n")
        .unwrap()
        .split(",")
        .fold(0, |acc, ran| {
            let values: Vec<u64> = ran.split("-").map(|x| x.parse::<u64>().unwrap()).collect();

            let digits0 = values[0].checked_ilog10().unwrap_or(0) + 1;
            let digits1 = values[1].checked_ilog10().unwrap_or(0) + 1;

            let mut n_repetitions = 2;

            let mut numbers: Vec<u64> = vec![];

            while n_repetitions <= digits1 {
                let mut potential_number = if digits0.is_multiple_of(n_repetitions) {
                    values[0] / 10.pow(digits0 - (digits0 / n_repetitions))
                } else {
                    10.pow(digits0 / n_repetitions)
                };

                while construct_repeated_digit(potential_number, n_repetitions) < values[0] {
                    potential_number += 1;
                }

                while construct_repeated_digit(potential_number, n_repetitions) <= values[1] {
                    numbers.push(construct_repeated_digit(potential_number, n_repetitions));
                    potential_number += 1;
                }
                n_repetitions += 1;
            }

            acc + numbers.iter().unique().sum::<u64>()
        })
}

/*
*
* 22 rep 2
*
* */

#[cfg(test)]
mod tests {
    use crate::y2025::day02::{construct_duplicated_digit, construct_repeated_digit, func1, func2};

    use rstest::rstest;
    #[test]
    fn test_duplicated_digit() {
        let input = 11885;
        let expected = 1188511885;

        assert_eq!(construct_duplicated_digit(input), expected);
    }

    #[rstest]
    #[case(11, (1, 2))]
    #[case(1111, (1, 4))]
    #[case(123123, (123, 2))]
    #[case(123123123, (123, 3))]
    #[case(1212121212, (12, 5))]
    #[case(12241224, (1224, 2))]
    fn test_repeated_digit(#[case] expected: u64, #[case] input: (u64, u32)) {
        assert_eq!(construct_repeated_digit(input.0, input.1), expected);
    }

    #[test]
    fn test_func1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";
        let expected = 1227775554;

        assert_eq!(func1(input), expected);
    }

    #[test]
    fn test_func2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";
        let expected = 4174379265;

        assert_eq!(func2(input), expected);
    }
}
