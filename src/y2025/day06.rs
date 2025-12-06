use std::time::Instant;

use nom::branch::alt;
use nom::combinator::map_res;
use nom::{
    bytes::streaming::tag, character::streaming::newline, multi::separated_list1,
    sequence::separated_pair, IResult,
};

use nom::character::complete::{self, space1};

use crate::utils::transposer;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

#[derive(Debug)]
enum Operation {
    Sum,
    Mul,
}

impl Operation {
    fn from_ch(c: char) -> Option<Self> {
        match c {
            '+' => Some(Operation::Sum),
            '*' => Some(Operation::Mul),
            _ => None,
        }
    }

    fn from_str(c: &str) -> Option<Self> {
        Self::from_ch(c.chars().next()?)
    }
}

fn parse_operands(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(newline, separated_list1(space1, complete::u64))(input)
}

fn parse_operations(input: &str) -> IResult<&str, Vec<Operation>> {
    separated_list1(
        space1,
        map_res(alt((tag("+"), tag("*"))), |s| {
            Operation::from_str(s).ok_or("invalid operation")
        }),
    )(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<Operation>)> {
    separated_pair(parse_operands, newline, parse_operations)(input)
}

#[tracing::instrument(skip(input))]
fn func1(input: &str) -> u64 {
    let (_, (operands, operations)) = parse(input).unwrap();

    let t_op = transposer::traspose_num_vec(operands);
    t_op.iter()
        .zip(operations)
        .map(|(nums, op)| match op {
            Operation::Sum => nums.iter().sum::<u64>(),
            Operation::Mul => nums.iter().product(),
        })
        .sum()
}

fn parse_col(lines: &Vec<&str>, col:usize) -> Option<u64>{

        lines
            .iter()
            .take(lines.len().saturating_sub(1))
            .filter_map(|line| line.chars().nth(col))
            .collect::<String>() // joined column
            //
            .trim()
            .parse()
            .ok()
}

fn parse2(input: &str) -> (Vec<Vec<u64>>, Vec<Operation>) {
    let mut blocks = vec![];
    let mut current_block: Vec<u64> = vec![];
    let mut operations = vec![];

    let lines: Vec<&str> = input.lines().collect();

    for (col, op) in lines.last().unwrap().chars().enumerate() {
        if op != ' ' {
            if !current_block.is_empty() {
                blocks.push(current_block);
            }
            current_block = vec![];
            operations.push(Operation::from_ch(op).unwrap());
        }

        if let Some(num) = parse_col(&lines, col) {
            current_block.push(num);
        }
    }
    blocks.push(current_block);
    (blocks, operations)
}

#[tracing::instrument(skip(input))]
fn func2(input: &str) -> u64 {
    let (operands, operations) = parse2(input);

    operands
        .iter()
        .zip(operations)
        .map(|(nums, op)| match op {
            Operation::Sum => nums.iter().sum::<u64>(),
            Operation::Mul => nums.iter().product(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_func1() {
        let input = "123 328  51 64
45 64  387 23
6 98  215 314
*   +   *   +
";
        let expected = 4277556;

        assert_eq!(func1(input), expected);
    }

    #[test]
    fn test_func2() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
        let expected = 3263827;

        assert_eq!(func2(input), expected);
    }
}
