use std::{collections::{HashMap, HashSet}, time::Instant};

use num::ToPrimitive;

use crate::utils::{coords::Coordinates, direction::Direction};

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", get_guard_path(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn get_guard_path(input: &str) -> usize {
    let mut map: Vec<String> = input.lines()
        .map(|line| line.to_string())
        .collect();

    let mut guard = get_intial_guard_position(&map);
    let mut visited : HashSet<Coordinates<u32>> = HashSet::new();
    let mut dir = Direction::Down;

    loop {
        visited.insert(guard);
        let co = match guard.try_step(dir) {
            Some(co) => co,
            None => break
        };

        let y: usize = co.y().try_into().unwrap();
        let row = match map.get(y) {
            None => break,
            Some(row) => row,
        };

        let x: usize = co.x().try_into().unwrap();
        let ch = match (*row).chars().nth(x) {
            None => break,
            Some(ch) => ch,
        };

        match ch {
            '#' => dir = dir.turn_left(),
            _ => {
                guard = co;
                map.get_mut(y).unwrap().replace_range(x..x + 1, "X");
            }
        };
    }

    visited.len()
}

fn get_intial_guard_position(map: &Vec<String>) -> Coordinates<u32> {
    map.iter()
            .enumerate()
            .find_map(|(i, line)| {
                match line.chars()
                    .enumerate()
                    .find_map(|(j, ch)| {
                        if ch == '^' { Some(j) } else { None }
                    }){
                    Some(j) => Some(Coordinates::new(j.to_u32().unwrap(), i.to_u32().unwrap())),
                    None => None
                }
            }).unwrap()
}


fn func2(input: &str) -> usize {
    let mut map: Vec<String> = input.lines()
        .map(|line| line.to_string())
        .collect();

    let mut guard = get_intial_guard_position(&map);
    let mut dir = Direction::Down;
    let mut obstacles = 0;

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(41, "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...")]
    fn test_func1(#[case] expected: usize, #[case] input: &str) {
        assert_eq!(expected, get_guard_path(input))
    }

    #[test]
    fn test_func2() {
        let expected = 6;
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        assert_eq!(expected, func2(input))
    }
}
