use rstest::rstest;
use std::{i128, time::Instant};

use crate::utils::parser;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", part_1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", part_2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn winning_times(time: i128, distance: i128) -> i128 {
    // milimiters_travelled = ms_pressed*(ms_pressed - time)
    //
    // -p^2 + tp - d > 0

    let square: f64 = ((time.pow(2) - 4 * distance) as f64).sqrt();

    let bound1 = -((-time as f64 + square) / 2.0);
    let bound2 = -((-time as f64 - square) / 2.0);

    let res1 = if bound1 == bound1.floor() {
        (bound1 + 1.0) as i128
    } else {
        (bound1.floor() + 1.0) as i128
    };
    let res2 = if bound2 == bound2.floor() {
        (bound2 - 1.0) as i128
    } else {
        bound2.floor() as i128
    };

    res2 - res1 + 1
}

fn part_1(input: &str) -> i128 {
    let times: Vec<i128> = parser::get_number_from_line(input.lines().next().unwrap());
    let distances: Vec<i128> = parser::get_number_from_line(input.lines().last().unwrap());

    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| winning_times(time, distance))
        .product()
}

fn part_2(input: &str) -> i128 {
    let trimmed_input = input.replace(" ", "");
    let time: i128 = parser::get_number_from_line(trimmed_input.lines().next().unwrap())[0];
    let distance: i128 = parser::get_number_from_line(trimmed_input.lines().last().unwrap())[0];

    winning_times(time, distance)
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
    fn test_func1(#[case] expected: i128, #[case] input: &str) {
        assert_eq!(expected, part_1(input));
    }

    #[test]
    fn test_func2() {
        let expected = 71503;
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(expected, part_2(input));
    }

    #[rstest]
    #[case(4, 7, 9)]
    #[case(8, 15, 40)]
    #[case(9, 30, 200)]
    fn test_winning_times(#[case] expected: i128, #[case] time: i128, #[case] distance: i128) {
        assert_eq!(expected, winning_times(time, distance))
    }
}
// primer intento
//
// fn num_of_ways_to_beat(time: u128, distance: u128) -> u128 {
//     range(0, time - 1)
//         .filter(|ms_pressed| ms_pressed * (time - ms_pressed) > distance)
//         .count()
//         .try_into()
//         .unwrap()
// }
