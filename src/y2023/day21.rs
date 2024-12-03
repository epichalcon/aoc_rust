use rstest::rstest;
use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

use crate::utils::coords::Coordinates;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input, 64));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func1(input, 26501365));
    println!("\t time:{:?}", start_time.elapsed());
}

fn get_start_position(input: &str) -> Coordinates<i32> {
    let (x, y) = input
        .lines()
        .enumerate()
        .find_map(|(y, line)| match line.find('S') {
            Some(x) => Some((x, y)),
            None => None,
        })
        .expect("S not found");

    Coordinates::new(x.try_into().unwrap(), y.try_into().unwrap())
}

fn func1(input: &str, steps: usize) -> usize {
    let mut q = VecDeque::<(Coordinates<i32>, usize)>::new();

    let mut even_coords = HashSet::<Coordinates<i32>>::new();
    let mut visited = HashSet::<Coordinates<i32>>::new();

    let starting_coord = get_start_position(input);

    q.push_back((starting_coord, 0));

    let mut i = 0;

    while !q.is_empty() {
        let (actual_coord, i) = q.pop_front().unwrap();

        let normalized_coord = normalice_coord(actual_coord, input);

        if get_char_at_coord(input, normalized_coord) == '#'
            || visited.contains(&actual_coord)
            || i > steps
        {
            continue;
        }

        for coord in actual_coord.orthogonal_neighbors() {
            q.push_back((coord, i + 1));
        }

        if i % 2 == steps % 2 {
            even_coords.insert(actual_coord.clone());
        }
        visited.insert(actual_coord.clone());
    }

    even_coords.len()
}

fn normalice_coord(coord: Coordinates<i32>, input: &str) -> Coordinates<i32> {
    let x = coord
        .x()
        .rem_euclid(input.lines().next().unwrap().len().try_into().unwrap());
    let y = coord
        .y()
        .rem_euclid(input.lines().count().try_into().unwrap());
    Coordinates::new(x, y)
}

fn get_char_at_coord(input: &str, position: Coordinates<i32>) -> char {
    input
        .lines()
        .nth(position.y() as usize)
        .unwrap()
        .chars()
        .nth(position.x() as usize)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func1() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

        assert_eq!(16, func1(input, 6));
        assert_eq!(50, func1(input, 10));
        assert_eq!(1594, func1(input, 50));
        assert_eq!(6536, func1(input, 100));
    }
}
