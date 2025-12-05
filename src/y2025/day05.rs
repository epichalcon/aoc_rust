use std::{time::Instant, vec};

use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    sequence::separated_pair, IResult,
};

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

#[derive(Debug)]
struct FreshIngredients {
    ranges: Vec<(u64, u64)>,
}

impl FreshIngredients {
    fn new() -> Self {
        Self { ranges: vec![] }
    }

    fn add_range(mut self, n_range: (u64, u64)) -> Self {
        self.ranges.push(n_range);
        self
    }

    fn contains(&self, ingredient: &u64) -> bool {
        self.ranges
            .iter()
            .any(|range| range.0 <= *ingredient && *ingredient <= range.1)
    }

    fn merge(&mut self) {
        if self.ranges.is_empty() {
            return;
        };

        self.ranges.sort_by(|a, b| a.0.cmp(&b.0));

        let mut merged = vec![self.ranges[0]];

        for current in &mut self.ranges[1..] {
            let last = *merged.last().unwrap();

            if current.0 <= last.1 {
                merged.pop();
                merged.push((last.0, u64::max(last.1, current.1)));
            } else {
                merged.push(*current);
            }
        }

        self.ranges = merged.to_vec();
    }

    fn len(&self) -> u64 {
        self.ranges
            .iter()
            .fold(0, |acc, range| range.1 - range.0 + 1 + acc)
    }
}

fn range_parser(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list1(
        tag("\n"),
        separated_pair(
            map_res(digit1, |s: &str| s.parse::<u64>()),
            tag("-"),
            map_res(digit1, |s: &str| s.parse::<u64>()),
        ),
    )(input)
}

fn ingredient_parser(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag("\n"), map_res(digit1, |s: &str| s.parse::<u64>()))(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<(u64, u64)>, Vec<u64>)> {
    separated_pair(range_parser, tag("\n\n"), ingredient_parser)(input)
}

#[tracing::instrument(skip(input))]
fn func1(input: &str) -> u64 {
    let (_, (ranges, ingredients)) = parse(input).unwrap();

    let fresh_ingredients: FreshIngredients = ranges
        .iter()
        .fold(FreshIngredients::new(), |acc, range| acc.add_range(*range));

    ingredients
        .iter()
        .filter(|ingredient| fresh_ingredients.contains(ingredient))
        .count() as u64
}

#[tracing::instrument(skip(input))]
fn func2(input: &str) -> u64 {
    let (_, ranges) = range_parser(input).unwrap();

    let mut fresh_ingredients: FreshIngredients = ranges
        .iter()
        .fold(FreshIngredients::new(), |acc, range| acc.add_range(*range));

    fresh_ingredients.merge();
    fresh_ingredients.len()
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
