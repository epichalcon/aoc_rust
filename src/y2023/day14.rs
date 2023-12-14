use rstest::rstest;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use crate::utils::transposer::{traspose_string, traspose_string_vec};

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", slide_once(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", slide_infinitely(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn slide_once(input: &str) -> usize {
    let processed_input = move_north(input.to_string());

    processed_input
        .lines()
        .enumerate()
        .map(|(i, line)| line.matches('O').count() * (processed_input.lines().count() - i))
        .sum()
}

fn slide_infinitely(input: &str) -> usize {
    let mut processed_input = input.to_string();

    let mut visited_states = HashMap::new();

    let mut i = 0;
    loop {
        processed_input = move_north(processed_input);
        processed_input = move_west(processed_input);
        processed_input = move_south(processed_input);
        processed_input = move_east(processed_input);
        i += 1;

        if visited_states.contains_key(&processed_input) {
            break;
        }
        visited_states.insert(processed_input.clone(), i);
    }

    let iterations_left = (1000000000 - i) % (i - visited_states.get(&processed_input).unwrap());

    i = 0;

    loop {
        processed_input = move_north(processed_input);
        processed_input = move_west(processed_input);
        processed_input = move_south(processed_input);
        processed_input = move_east(processed_input);

        i += 1;
        if i == iterations_left {
            break;
        }
    }
    dbg!(i);

    processed_input
        .lines()
        .enumerate()
        .map(|(i, line)| line.matches('O').count() * (processed_input.lines().count() - i))
        .sum()
}

fn move_east(processed_input: String) -> String {
    move_down(processed_input.lines().collect()).join("\n")
}

fn move_south(processed_input: String) -> String {
    let trasposed_input = traspose_string(&processed_input);

    traspose_string_vec(move_down(trasposed_input.lines().collect()))
}

fn move_west(processed_input: String) -> String {
    move_up(processed_input.lines().collect()).join("\n")
}

fn move_north(input: String) -> String {
    let trasposed_input = traspose_string(&input);

    traspose_string_vec(move_up(trasposed_input.lines().collect()))
}

fn move_down(input: Vec<&str>) -> Vec<String> {
    input.iter().fold(Vec::new(), |mut acc, line| {
        let new_line = move_up_stones(line.chars().rev().collect::<String>());
        acc.push(new_line.chars().rev().collect::<String>());
        acc
    })
}

fn move_up(input: Vec<&str>) -> Vec<String> {
    input.iter().fold(Vec::new(), |mut acc, line| {
        let new_line = move_up_stones(line.to_string());
        acc.push(new_line);
        acc
    })
}

fn move_up_stones(line: String) -> String {
    let mut new_line_vec: Vec<char> = vec![];

    for (i, ch) in line.chars().enumerate() {
        if ch == 'O' {
            new_line_vec.push('O');
        }
        if ch == '#' {
            let empty_spaces = i - new_line_vec.len();
            new_line_vec.append(&mut vec!['.'; empty_spaces].clone());
            new_line_vec.push('#')
        }
    }

    let empty_spaces = line.len() - new_line_vec.len();

    new_line_vec.append(&mut vec!['.'; empty_spaces].clone());

    new_line_vec.iter().collect()
}

fn func2(input: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[rstest]
    // #[case()]
    // fn test_func1(#[case] expected: u32, #[case] input: &str) {}

    #[test]
    fn test_func1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let expected = 136;
        assert_eq!(expected, slide_once(input));
    }

    #[test]
    fn test_func2() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let expected = 64;
        assert_eq!(expected, slide_infinitely(input));
    }
}
