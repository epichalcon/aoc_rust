use rstest::rstest;
use std::{i32, mem::Discriminant, time::Instant};

use crate::utils::{coords::Coordinates, direction::Direction};

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", calculate_area(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", calculate_area_hex(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn calculate_area(input: &str) -> i128 {
    let mut coord = Coordinates::new(0, 0);
    let mut res = 0;
    let mut dim = 1;
    for line in input.lines() {
        let (direction, steps) = get_direction_steps(line);

        let new_coord = coord.step_by(direction, steps);

        if direction == Direction::Down || direction == Direction::Left {
            dim += steps
        }

        res += calculate_pair(coord, new_coord);
        coord = new_coord;
    }

    res.abs() / 2 + dim
}

fn calculate_pair(coord1: Coordinates<i128>, coord2: Coordinates<i128>) -> i128 {
    coord1.x() * coord2.y() - (coord1.y() * coord2.x())
}

fn get_direction_steps(input: &str) -> (Direction, i128) {
    let mut split_it = input.split(" ");

    let direction = match split_it.next().unwrap() {
        "R" => Direction::Right,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "U" => Direction::Up,
        _ => panic!(),
    };
    let steps = split_it.next().unwrap().parse::<i128>().unwrap();

    (direction, steps)
}

fn calculate_area_hex(input: &str) -> i128 {
    let mut coord = Coordinates::new(0, 0);
    let mut res = 0;
    let mut dim = 1;
    for line in input.lines() {
        let (direction, steps) = get_direction_steps_from_hex(line);

        let new_coord = coord.step_by(direction, steps);

        if direction == Direction::Down || direction == Direction::Left {
            dim += steps
        }

        res += calculate_pair(coord, new_coord);
        coord = new_coord;
    }

    res.abs() / 2 + dim
}

fn get_direction_steps_from_hex(line: &str) -> (Direction, i128) {
    let hex = line.split("#").last().unwrap();

    let steps = i128::from_str_radix(&hex[0..5], 16).unwrap();

    let direction = match &hex[5..6] {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        ch => panic!("paniked with char {}", ch),
    };

    (direction, steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_area_small() {
        let input = "R 1 (#70c710)
D 1 (#70c710)
L 1 (#70c710)
U 1 (#7a21e3)";
        let res = 4;

        assert_eq!(res, calculate_area(input));
    }

    #[test]
    fn test_calculate_area() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let res = 62;

        assert_eq!(res, calculate_area(input));
    }

    #[test]
    fn test_calculate_area_hex() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let res = 952408144115;

        assert_eq!(res, calculate_area_hex(input));
    }
}
