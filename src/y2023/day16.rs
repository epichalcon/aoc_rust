use crate::utils::{coords::Coordinates, direction::Direction};
use rstest::rstest;
use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn func1(input: &str) -> usize {
    calculate_energy(input, Coordinates::new(0, 0), Direction::Right)
}

fn calculate_energy(
    input: &str,
    starting_coord: Coordinates<i32>,
    starting_direction: Direction,
) -> usize {
    let mut energized = HashSet::new();
    let mut q: VecDeque<(Coordinates<i32>, Direction)> = VecDeque::new();
    q.push_back((starting_coord, starting_direction));

    let mut visited_mirrors = HashSet::new();

    while !q.is_empty() {
        if let Some((coord, direction)) = q.pop_front() {
            if !is_in_bounds(coord, input) {
                continue;
            }
            let ch = input
                .lines()
                .nth(coord.y() as usize)
                .unwrap()
                .chars()
                .nth(coord.x() as usize)
                .unwrap();

            if ch != '.' {
                if visited_mirrors.contains(&(coord, direction)) {
                    continue;
                }

                visited_mirrors.insert((coord, direction));
            }
            energized.insert(coord);
            let (new_direction, optional_direction) = get_next_direction(ch, direction);
            q.push_back((coord.step(new_direction), new_direction));
            match optional_direction {
                Some(optional) => q.push_back((coord.step(optional), optional)),
                None => (),
            };
        }
    }
    energized.len()
}

fn is_in_bounds(coord: Coordinates<i32>, input: &str) -> bool {
    !(coord.x() < 0
        || coord.y() < 0
        || coord.x() >= input.lines().next().unwrap().len().try_into().unwrap()
        || coord.y() >= input.lines().next().unwrap().len().try_into().unwrap())
}

fn get_next_direction(ch: char, direction: Direction) -> (Direction, Option<Direction>) {
    match direction {
        Direction::Up => match ch {
            '|' | '.' => (Direction::Up, None),
            '\\' => (Direction::Left, None),
            '/' => (Direction::Right, None),
            '-' => (Direction::Right, Some(Direction::Left)),
            c => panic!("incorrect direction: {}", c),
        },
        Direction::Down => match ch {
            '|' | '.' => (Direction::Down, None),
            '/' => (Direction::Left, None),
            '\\' => (Direction::Right, None),
            '-' => (Direction::Right, Some(Direction::Left)),
            c => panic!("incorrect direction: {}", c),
        },
        Direction::Left => match ch {
            '-' | '.' => (Direction::Left, None),
            '/' => (Direction::Down, None),
            '\\' => (Direction::Up, None),
            '|' => (Direction::Up, Some(Direction::Down)),
            c => panic!("incorrect direction: {}", c),
        },
        Direction::Right => match ch {
            '-' | '.' => (Direction::Right, None),
            '/' => (Direction::Up, None),
            '\\' => (Direction::Down, None),
            '|' => (Direction::Up, Some(Direction::Down)),
            c => panic!("incorrect direction: {}", c),
        },
    }
}

fn func2(input: &str) -> usize {
    let col_1 = (0..input.lines().count())
        .map(|y| {
            calculate_energy(
                input,
                Coordinates::new(0, y.try_into().unwrap()),
                Direction::Right,
            )
        })
        .max()
        .unwrap();
    let col_2 = (0..input.lines().count())
        .map(|y| {
            calculate_energy(
                input,
                Coordinates::new(
                    (input.lines().next().unwrap().len() - 1)
                        .try_into()
                        .unwrap(),
                    y.try_into().unwrap(),
                ),
                Direction::Left,
            )
        })
        .max()
        .unwrap();

    let row_1 = (0..input.lines().next().unwrap().len())
        .map(|x| {
            calculate_energy(
                input,
                Coordinates::new(x.try_into().unwrap(), 0),
                Direction::Right,
            )
        })
        .max()
        .unwrap();

    let row_2 = (0..input.lines().next().unwrap().len())
        .map(|x| {
            calculate_energy(
                input,
                Coordinates::new(
                    x.try_into().unwrap(),
                    (input.lines().count() - 1).try_into().unwrap(),
                ),
                Direction::Left,
            )
        })
        .max()
        .unwrap();

    *vec![row_2, row_1, col_2, col_1].iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func2() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        let expected = 46;

        assert_eq!(expected, func1(input));
    }
}
