use std::{ops::RangeInclusive, time::Instant};

use itertools::Itertools;
use nom::{
    bytes::complete::{tag},
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use rangemap::RangeInclusiveSet;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn range_parser(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(
        newline,
        separated_pair(
            complete::u64,
            tag("-"),
            complete::u64,
        )
        .map(|(a, b)| a..=b),
    )(input)
}

fn ingredient_parser(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag("\n"), complete::u64)(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    separated_pair(range_parser, newline.and(newline), ingredient_parser)(input)
}

#[tracing::instrument(skip(input))]
fn func1(input: &str) -> u64 {
    let (_, (ranges, ingredients)) = parse(input).unwrap();

    let fresh_ingredients: RangeInclusiveSet<u64> = ranges.iter().cloned().collect();

    ingredients
        .iter()
        .filter(|ingredient| fresh_ingredients.contains(ingredient))
        .count() as u64
}

#[tracing::instrument(skip(input))]
fn func2(input: &str) -> u64 {
    let (_, ranges) = range_parser(input).unwrap();

    let fresh_ingredients: RangeInclusiveSet<u64> = ranges.iter().cloned().collect();

    fresh_ingredients
        .iter()
        .fold(0, |acc, range| range.try_len().unwrap() as u64 + acc)
}

#[cfg(test)]
mod tests {
    use super::*;
    //use rstest::rstest;
    use test_log::test;

    /*
    #[rstest]
    #[case()]
    fn test_func1(#[case] expected: u64, #[case] input: &str) {}
    */

    #[test]
    fn test_func1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let expected = 3;
        assert_eq!(func1(input), expected);
    }

    #[test]
    fn test_func2() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let expected = 14;
        assert_eq!(func2(input), expected);
    }
}
