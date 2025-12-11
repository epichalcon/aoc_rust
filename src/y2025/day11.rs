use core::str;
use std::{collections::HashMap, time::Instant};

use glam::usize;
use nom::{
    bytes::complete::tag,
    character::{
        self,
        complete::{alpha1, newline, space1},
    },
    multi::separated_list1,
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

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    separated_pair(alpha1, tag(": "), separated_list1(space1, alpha1))(input)
}

fn parse(input: &str) -> IResult<&str, HashMap<&str, Vec<&str>>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines.into_iter().collect()))
}

#[tracing::instrument(skip(input))]
fn func1(input: &str) -> usize {
    let (_, outputs) = parse(input).expect("unable to parse");

    count_paths(
        "you",
        |node| {
            outputs
                .get(node)
                .expect("node should be in map")
                .iter()
                .cloned()
        },
        |n| n == &"out",
    )
}

#[tracing::instrument(skip(input))]
fn func2(input: &str) -> usize {
    let (_, outputs) = parse(input).expect("unable to parse");


    let empty_vec = vec![];

    let svr_to_fft = count_paths(
        "svr",
        |node| {
            outputs
                .get(node)
                .unwrap_or(&empty_vec)
                .iter()
                .copied()
        },
        |n| n == &"fft",
    );

    let fft_to_dac = count_paths(
        "fft",
        |node| {
            outputs
                .get(node)
                .unwrap_or(&empty_vec)
                .iter()
                .copied()
        },
        |n| n == &"dac",
    );

    let dac_to_out = count_paths(
        "dac",
        |node| {
            outputs
                .get(node)
                .unwrap_or(&empty_vec)
                .iter()
                .copied()
        },
        |n| n == &"out",
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
