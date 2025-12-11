use core::str;
use std::{collections::HashMap, time::Instant};

use glam::usize;
use nom::{
    bytes::complete::tag,
    character::{
        self,
        complete::{alpha1, newline},
    },
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use pathfinding::prelude::count_paths;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn parse_name(input: &str) -> IResult<&str, String> {
    let (input, name) = many1(alpha1)(input)?;
    Ok((input, name.iter().cloned().collect::<String>()))
}

fn parse_line(input: &str) -> IResult<&str, (String, Vec<String>)> {
    separated_pair(
        parse_name,
        tag(": "),
        separated_list1(character::complete::char(' '), parse_name),
    )(input)
}

fn parse(input: &str) -> IResult<&str, HashMap<String, Vec<String>>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((
        input,
        lines
            .iter()
            .cloned()
            .collect::<HashMap<String, Vec<String>>>(),
    ))
}

#[tracing::instrument(skip(input))]
fn func1(input: &str) -> usize {
    let (_, outputs) = parse(input).expect("unable to parse");

    count_paths(
        "you".to_string(),
        |node| {
            outputs
                .get(node.as_str())
                .expect("node should be in map")
                .iter()
                .cloned()
        },
        |n| n == &"out".to_string(),
    )
}

#[tracing::instrument(skip(input))]
fn func2(input: &str) -> usize {
    let (_, mut outputs) = parse(input).expect("unable to parse");

    outputs.insert("out".to_string(), vec![]);

    let svr_to_fft = count_paths(
        "svr".to_string(),
        |node| {
            outputs
                .get(node.as_str())
                .expect("node should be in map")
                .iter()
                .cloned()
        },
        |n| n == &"fft".to_string(),
    );

    let fft_to_dac = count_paths(
        "fft".to_string(),
        |node| {
            outputs
                .get(node.as_str())
                .expect("node should be in map")
                .iter()
                .cloned()
        },
        |n| n == &"dac".to_string(),
    );

    let dac_to_out = count_paths(
        "dac".to_string(),
        |node| {
            outputs
                .get(node.as_str())
                .expect("node should be in map")
                .iter()
                .cloned()
        },
        |n| n == &"out".to_string(),
    );

    svr_to_fft * fft_to_dac * dac_to_out
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_func1() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let expected = 5;

        assert_eq!(func1(input), expected);
    }

    #[test]
    fn test_func2() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";
        let expected = 2;

        assert_eq!(func2(input), expected);
    }
}
