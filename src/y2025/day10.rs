use core::panic;
use std::{collections::{HashSet, VecDeque}, time::Instant};

use glam::{u32, usize};
use nom::{
    character::{
        self,
        complete::{self, char, newline, one_of},
    },
    multi::{many1, separated_list0, separated_list1},
    sequence::delimited,
    IResult, 
};
use tracing::info;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct LightDiagram {
    diagram: Vec<bool>,
}

impl LightDiagram {
    fn new(lights: Vec<char>) -> Self {
        Self {
            diagram: lights
                .iter()
                .map(|l| match l {
                    '.' => false,
                    '#' => true,
                    _ => panic!("not a valid character"),
                })
                .collect(),
        }
    } 

    fn default(len: usize) -> Self {
        Self {
            diagram: vec![false; len],
        }
        
    }

    fn press_button(&mut self, button: &Vec<u32>) {
        for pos in button {
            let pos = *pos as usize;
            if pos > self.diagram.len() {
                panic!("button index out of bounds")
            }
            self.diagram[pos] = !self.diagram[pos]
        }
    }

    fn len(&self) -> usize {
        self.diagram.len()
    }
}

fn lights_parser(input: &str) -> IResult<&str, LightDiagram> {
    let (input, lights) = delimited(char('['), many1(one_of(".#")), char(']'))(input)?;
    Ok((input, LightDiagram::new(lights)))
}

fn button_parser(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    delimited(
        char(' '),
        separated_list1(
            char(' '),
            delimited(
                char('('),
                separated_list0(char(','), character::complete::u32),
                char(')'),
            ),
        ),
        char(' '),
    )(input)
}

fn joltage_parser(input: &str) -> IResult<&str, Vec<u32>> {
    delimited(
        char('{'),
        separated_list0(char(','), character::complete::u32),
        char('}'),
    )(input)
}

fn machine_parser(input: &str) -> IResult<&str, (LightDiagram, Vec<Vec<u32>>, Vec<u32>)> {
    let (input, lights) = lights_parser(input)?;
    let (input, buttons) = button_parser(input)?;
    let (input, joltage) = joltage_parser(input)?;

    Ok((input, (lights, buttons, joltage)))
}

fn parse(input: &str) -> IResult<&str, Vec<(LightDiagram, Vec<Vec<u32>>, Vec<u32>)>> {
    separated_list1(newline, machine_parser)(input)
}

fn get_buttons_pressed(objective_lights: LightDiagram, buttons: Vec<Vec<u32>>) -> u32 {

    let mut visited : HashSet<LightDiagram> = HashSet::new();
    let mut queue : VecDeque<(LightDiagram, &Vec<u32>, u32)> = VecDeque::new();

    for new_button in &buttons {
        queue.push_back((LightDiagram::default(objective_lights.len()), new_button, 0));
    }

    visited.insert(LightDiagram::default(objective_lights.len()));


    while let Some((mut lights, button, mut pressed_before)) = queue.pop_front() {
        lights.press_button(button);
        pressed_before += 1;

        if visited.contains(&lights) {
            continue;
        }
        visited.insert(lights.clone());

        if  lights == objective_lights {
            return pressed_before;
        }

        for new_button in &buttons {
            queue.push_back((lights.clone(), new_button, pressed_before));
        }
    }
    panic!("should not be reachable");
}



#[tracing::instrument(skip(input))]
fn func1(input: &str) -> u32 {
    let (_, machines) = parse(input).expect("parsing failed");
    machines
        .iter()
        .map(|(lights, buttons, _)| get_buttons_pressed(lights.clone(), buttons.to_vec()))
        .sum()
}

#[tracing::instrument(skip(input))]
fn func2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use test_log::test;

    /*
    #[rstest]
    #[case()]
    fn test_func1(#[case] expected: u32, #[case] input: &str) {}
    */

    #[test]
    fn test_func1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let expected = 7;

        assert_eq!(func1(input), expected);
    }

    #[test]
    fn test_func2() {
        let input = "";
        let expected = 0;

        //assert_eq!(func2(input), expected);
    }
}
