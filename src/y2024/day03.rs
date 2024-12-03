use nom::{branch::alt, bytes::complete::{is_not, tag, take_while}, character::complete::{anychar, digit1, u32}, combinator::value, multi::{many0, many_till}, sequence::{delimited, preceded, separated_pair}, IResult, Parser};
use rstest::rstest;
use std::time::Instant;


pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", execute_muls(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", execute_muls_and_conditionals(input));
    println!("\t time:{:?}", start_time.elapsed());
}

//first star/////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
enum Instr {
    Mul(u32, u32),
    Do,
    Dont,
}

fn execute_muls(input: &str) -> u32 {
    extract_muls(input).unwrap().1
        .iter()
        .map(|mul| {
            match mul {
                Instr::Mul(a, b) => a*b,
                _ => panic!() // in this star we only consider mul
            }
        })
        .sum::<u32>()
}

fn extract_muls(input: &str) -> IResult<&str, Vec<Instr>> {
    Ok(many0(
        many_till(
            anychar,
            mul
        ).map(|(_, tup)| tup)
    )(input)?)
}


fn mul(input: &str) -> IResult<&str, Instr> {   // mul(a,b)
    let (input, (a, b)) = delimited(
        tag("mul("),
        separated_pair(digit1, tag(","), digit1),
        tag(")")
    )(input)?;
    Ok((input, Instr::Mul(a.parse().unwrap(), b.parse().unwrap())))
}


//second star/////////////////////////////////////////////////////////////

struct State {
    enabled: bool,
    sum: u32,
}

impl State {
    fn new() -> Self{
        Self {
            enabled: true,
            sum: 0
        }
    }

    fn get_sum(&self) -> u32 {
        self.sum
    }

    fn enable(&mut self) -> () {
        self.enabled = true;
    }

    fn disable(&mut self) -> () {
        self.enabled = false;
    }

    fn add(&mut self, amount: u32) -> () {
        if self.enabled {
            self.sum += amount
        }
    }
}

fn execute_muls_and_conditionals(input: &str) -> u32 {
    extract_muls_and_conditionals(input).unwrap().1
        .iter()
        .fold(State::new(), |mut state, instr| {
            match instr {
                Instr::Do => state.enable(),
                Instr::Dont => state.disable(),
                Instr::Mul(a, b) => state.add(a*b),
            }
            state
        })
        .get_sum()
}

fn extract_muls_and_conditionals(input: &str) -> IResult<&str, Vec<Instr>> {
    Ok(many0(
        many_till(
            anychar,
            alt((
                value(Instr::Do, tag("do()")),
                value(Instr::Dont, tag("don't()")),
                mul
            ))
        ).map(|(_, tup)| tup)
    )(input)?)
}


//tests///////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(161, "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")]
    fn test_func1(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, execute_muls(input))
    }

    #[rstest]
    #[case(48, "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")]
    fn test_func2(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, execute_muls_and_conditionals(input))
    }
}
