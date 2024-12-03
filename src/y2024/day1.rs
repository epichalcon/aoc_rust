use rstest::rstest;
use std::{collections::HashMap, hash::Hash, iter::zip, time::Instant, u32};

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", distances(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", similarity(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn difference(a:u32, b:u32) -> u32 {
    if a > b {
        a - b
    }
    else{
        b - a
    }
}

fn distances(input: &str) -> u32 {
    let mut parsed_input: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            let splitted : Vec<u32> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            splitted
        })
        .fold(vec![Vec::new(), Vec::new()], |mut acc, splitted| {
            if !splitted.is_empty(){
                acc[0].push(splitted[0]);
                acc[1].push(splitted[1]);
            }
            acc}
        );
    parsed_input[0].sort();
    parsed_input[1].sort();

    zip(
        parsed_input[0].clone(),
        parsed_input[1].clone())
        .map(|(a, b)| difference(a,b))
        .sum()
}
fn similarity(input: &str) -> u32 {
    let mut parsed_input: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            let splitted : Vec<u32> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            splitted
        })
        .fold(vec![Vec::new(), Vec::new()], |mut acc, splitted| {
            if !splitted.is_empty(){
                acc[0].push(splitted[0]);
                acc[1].push(splitted[1]);
            }
            acc}
        );

    let mut counts : HashMap<u32, u32> = HashMap::new();
    for &number in &parsed_input[1] {
        *counts.entry(number).or_insert(0) += 1;
    }

    parsed_input[0].iter().fold(0, |acc, num| counts.get(num).unwrap_or(&0) * num + acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(11, "3   4
4   3
2   5
1   3
3   9
3   3")]

    fn test_func1(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, distances(input));
    }

    #[rstest]
    #[case(31, "3   4
4   3
2   5
1   3
3   9
3   3")]

    fn test_func2(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, similarity(input));
    }
}
