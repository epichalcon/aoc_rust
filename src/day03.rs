use core::panic;
use std::{collections::HashSet, time::Instant};

use crate::utils::coords::Coordinates;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", houses_with_presents(&input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", robot_christmas(&input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn houses_with_presents(intput: &str) -> usize {
    let mut houses = HashSet::<Coordinates<i32>>::new();

    let mut position = Coordinates::origin();
    houses.insert(position);

    for c in intput.chars() {
        match c {
            '>' => position = position.right(),
            '<' => position = position.left(),
            '^' => position = position.up(),
            'v' => position = position.down(),
            '\n' => (),
            ch => panic!("Direction not recognized '{}'", ch),
        }
        houses.insert(position);
    }

    houses.len()
}

fn robot_christmas(intput: &str) -> usize {
    let mut houses = HashSet::<Coordinates<i32>>::new();

    let mut santas_pos = Coordinates::origin();
    let mut robots_pos = Coordinates::origin();

    houses.insert(santas_pos);

    let mut santas_turn = true;

    for c in intput.chars() {
        match c {
            '>' => {
                if santas_turn {
                    santas_pos = santas_pos.right();
                } else {
                    robots_pos = robots_pos.right();
                }
            }
            '<' => {
                if santas_turn {
                    santas_pos = santas_pos.left();
                } else {
                    robots_pos = robots_pos.left();
                }
            }
            '^' => {
                if santas_turn {
                    santas_pos = santas_pos.up();
                } else {
                    robots_pos = robots_pos.up();
                }
            }
            'v' => {
                if santas_turn {
                    santas_pos = santas_pos.down();
                } else {
                    robots_pos = robots_pos.down();
                }
            }
            '\n' => (),
            ch => panic!("Direction not recognized '{}'", ch),
        }

        if santas_turn {
            houses.insert(santas_pos);
            santas_turn = false;
        } else {
            houses.insert(robots_pos);
            santas_turn = true;
        }
    }

    houses.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_houses_with_presents_beguining() {
        assert_eq!(1, houses_with_presents(""));
    }

    #[test]
    fn test_houses_with_presents_all_new() {
        assert_eq!(2, houses_with_presents(">"));
        assert_eq!(6, houses_with_presents(">^<<v"));
    }

    #[test]
    fn test_houses_with_presents_repete() {
        assert_eq!(2, houses_with_presents(">"));
        assert_eq!(4, houses_with_presents("^>v<"));
        assert_eq!(2, houses_with_presents("^v^v^v^v^v"));
    }

    #[test]
    fn test_robot_chirstmas0() {
        assert_eq!(2, robot_christmas("^"));
    }
    #[test]
    fn test_robot_chirstmas1() {
        assert_eq!(3, robot_christmas("^v"));
    }
    #[test]
    fn test_robot_chirstmas2() {
        assert_eq!(3, robot_christmas("^>v<"));
    }
    #[test]
    fn test_robot_chirstmas3() {
        assert_eq!(11, robot_christmas("^v^v^v^v^v"));
    }
}
