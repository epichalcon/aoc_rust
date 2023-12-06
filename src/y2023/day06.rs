use num::range;
use rstest::rstest;
use std::{time::Instant, u128};

use crate::utils::parser;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", part_1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", part_2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn part_1(input: &str) -> u128 {
    let times: Vec<u32> = parser::get_number_from_line(input.lines().next().unwrap());
    let distances: Vec<u32> = parser::get_number_from_line(input.lines().last().unwrap());

    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| num_of_ways_to_beat(time as u128, distance as u128))
        .product()
}

fn num_of_ways_to_beat(time: u128, distance: u128) -> u128 {
    // milimiters_travelled = ms_pressed*(ms_pressed - time)
    range(0, time - 1)
        .filter(|ms_pressed| ms_pressed * (time - ms_pressed) > distance)
        .count()
        .try_into()
        .unwrap()
}

fn part_2(input: &str) -> u128 {
    let trimmed_input = input.replace(" ", "");
    let time = parser::get_u128_from_line(trimmed_input.lines().next().unwrap())[0];
    let distance = parser::get_u128_from_line(trimmed_input.lines().last().unwrap())[0];

    num_of_ways_to_beat(time as u128, distance as u128)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(
        4,
        "Time:      7
        Distance:  9"
    )]
    #[case(
        8,
        "Time:      15
        Distance:  40"
    )]
    #[case(
        9,
        "Time:      30
        Distance:  200"
    )]
    #[case(
        288,
        "Time:      7  15   30
Distance:  9  40  200"
    )]
    fn test_func1(#[case] expected: u128, #[case] input: &str) {
        assert_eq!(expected, part_1(input));
    }

    #[test]
    fn test_func2() {
        let expected = 71503;
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(expected, part_2(input));
    }
}
