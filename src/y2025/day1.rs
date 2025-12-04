use core::panic;
use std::time::Instant;

use nom::character::complete::none_of;

#[derive(Debug, Clone)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Rotation {
    direction: Dir,
    times: u32,
}

impl Rotation {
    fn new(rot: &str) -> Self {
        let (direction, times) = self::Rotation::extract_split_once(rot);

        Self { direction, times }
    }

    fn extract_split_once(s: &str) -> (Dir, u32) {
        let (letter, number_str) = s.split_at(1);
        let letter_char = letter.chars().next().unwrap();

        let dir = match letter_char {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!("not a valid rotation"),
        };

        let number = number_str.parse::<u32>().ok().unwrap();
        (dir, number)
    }
}

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn func1(input: &str) -> u32 {
    input
        .lines()
        .fold((50, 0), |(pos, zero_count), rotation| {
            let rot = Rotation::new(rotation);

            let n_pos = match rot.direction {
                Dir::Left => (pos as i32 - rot.times as i32).rem_euclid(100) as u32,
                Dir::Right => (pos + rot.times) % 100,
            };

            if n_pos == 0 {
                (n_pos, zero_count + 1)
            } else {
                (n_pos, zero_count)
            }
        })
        .1
}

fn func2(input: &str) -> u32 {
    input
        .lines()
        .fold((50, 0), |(pos, zero_count), rotation| {
            let rot = Rotation::new(rotation);

            let mut passed_through = rot.times / 100;
            let n_pos = match rot.direction {
                Dir::Left => {
                    let n_pos = (pos as i32 - rot.times as i32).rem_euclid(100) as u32;
                    if (n_pos > pos && pos != 0) || n_pos == 0 {
                        passed_through += 1;
                    }
                    n_pos
                }
                Dir::Right => {
                    let n_pos = (pos + rot.times) % 100;
                    if n_pos < pos {
                        passed_through += 1;
                    }
                    n_pos
                }
            };

            (n_pos, zero_count + passed_through)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        let expected = 3;
        assert_eq!(func1(input), expected);
    }

    #[test]
    fn test_func2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        let expected = 6;
        assert_eq!(func2(input), expected);
    }
}
