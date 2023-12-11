use rstest::rstest;
use std::{
    collections::{HashSet, VecDeque},
    ops::Index,
    time::Instant,
};

use crate::utils::{coords::Coordinates, direction::Direction};

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn get_start_position(input: &str) -> Coordinates<i16> {
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

fn get_start_direction(input: &str, position: Coordinates<i16>) -> Direction {
    let left_position = position.step(Direction::Left);

    if left_position.x() >= 0
        && left_position.y() >= 0
        && (get_char_at_coord(input, left_position) == '-'
            || get_char_at_coord(input, left_position) == 'F'
            || get_char_at_coord(input, left_position) == 'L')
    {
        Direction::Left
    } else if get_char_at_coord(input, position.step(Direction::Right)) == '-'
        || get_char_at_coord(input, position.step(Direction::Right)) == 'J'
        || get_char_at_coord(input, position.step(Direction::Right)) == '7'
    {
        Direction::Right
    } else if get_char_at_coord(input, position.step(Direction::Up)) == '|'
        || get_char_at_coord(input, position.step(Direction::Up)) == 'F'
        || get_char_at_coord(input, position.step(Direction::Up)) == '7'
    {
        Direction::Up
    } else {
        Direction::Down
    }
}

fn get_next_direction(pipe: char, direction: Direction) -> Direction {
    match direction {
        Direction::Up => match pipe {
            '|' => Direction::Up,
            '7' => Direction::Left,
            'F' => Direction::Right,
            c => panic!("incorrect direction: {}", c),
        },
        Direction::Down => match pipe {
            '|' => Direction::Down,
            'J' => Direction::Left,
            'L' => Direction::Right,
            c => panic!("incorrect direction: {}", c),
        },
        Direction::Left => match pipe {
            '-' => Direction::Left,
            'F' => Direction::Down,
            'L' => Direction::Up,
            c => panic!("incorrect direction: {}", c),
        },
        Direction::Right => match pipe {
            'J' => Direction::Up,
            '7' => Direction::Down,
            '-' => Direction::Right,
            c => panic!("incorrect direction: {}", c),
        },
    }
}

fn get_char_at_coord(input: &str, position: Coordinates<i16>) -> char {
    input
        .lines()
        .nth(position.y() as usize)
        .unwrap()
        .chars()
        .nth(position.x() as usize)
        .unwrap()
}

fn func1(input: &str) -> i32 {
    let mut position = get_start_position(input);

    let mut dir = get_start_direction(input, position);

    let mut i = 0;

    loop {
        i += 1;
        position = position.step(dir);

        let ch = get_char_at_coord(input, position);
        if ch == 'S' {
            break;
        }

        dir = get_next_direction(ch, dir);
    }

    i / 2
}

fn print_matrix(matrix: Vec<Vec<char>>) {
    for row in matrix.iter() {
        for ch in row.iter() {
            print!("{ch}");
        }
        println!("");
    }
}

fn func2(input: &str) -> usize {
    let mut expanded_map =
        vec![vec!['.'; input.lines().next().unwrap().len() * 3]; input.lines().count() * 3];

    let mut filled_positions = HashSet::new();

    let mut total_positions = 0;

    for (y, row) in input.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            total_positions += 1;

            match ch {
                'S' => {
                    filled_positions.insert((x as i32, y as i32));
                    expanded_map[(y * 3) + 0][(x * 3) + 0] = '#';
                    expanded_map[(y * 3) + 0][(x * 3) + 1] = '#';
                    expanded_map[(y * 3) + 0][(x * 3) + 2] = '#';
                    expanded_map[(y * 3) + 1][(x * 3) + 0] = '#';
                    expanded_map[(y * 3) + 1][(x * 3) + 1] = '#';
                    expanded_map[(y * 3) + 1][(x * 3) + 2] = '#';
                    expanded_map[(y * 3) + 2][(x * 3) + 0] = '#';
                    expanded_map[(y * 3) + 2][(x * 3) + 1] = '#';
                    expanded_map[(y * 3) + 2][(x * 3) + 2] = '#';
                }
                '-' => {
                    expanded_map[(y * 3) + 1][(x * 3) + 0] = '#';
                    expanded_map[(y * 3) + 1][(x * 3) + 1] = '#';
                    expanded_map[(y * 3) + 1][(x * 3) + 2] = '#';
                }
                '|' => {
                    expanded_map[(y * 3) + 0][(x * 3) + 1] = '#';
                    expanded_map[(y * 3) + 1][(x * 3) + 1] = '#';
                    expanded_map[(y * 3) + 2][(x * 3) + 1] = '#';
                }
                'J' => {
                    expanded_map[(y * 3) + 0][(x * 3) + 1] = '#';
                    expanded_map[(y * 3) + 1][(x * 3) + 1] = '#';
                    expanded_map[(y * 3) + 1][(x * 3) + 0] = '#';
                }
                'L' => {
                    expanded_map[(y * 3) + 0][(x * 3) + 1] = '#';
                    expanded_map[(y * 3) + 1][(x * 3) + 1] = '#';
                    expanded_map[(y * 3) + 1][(x * 3) + 2] = '#';
                }
                'F' => {
                    expanded_map[(y * 3) + 2][(x * 3) + 1] = '#';
                    expanded_map[(y * 3) + 1][(x * 3) + 1] = '#';
                    expanded_map[(y * 3) + 1][(x * 3) + 2] = '#';
                }
                '7' => {
                    expanded_map[(y * 3) + 2][(x * 3) + 1] = '#';
                    expanded_map[(y * 3) + 1][(x * 3) + 1] = '#';
                    expanded_map[(y * 3) + 1][(x * 3) + 0] = '#';
                }
                _ => {
                    expanded_map[y * 3][x * 3] = '.';
                }
            };
        }
    }

    let mut q: VecDeque<Coordinates<i32>> = VecDeque::new();
    q.push_back(Coordinates::new(0, 0));

    while !q.is_empty() {
        if let Some(coord) = q.pop_front() {
            if expanded_map[coord.y() as usize][coord.x() as usize] == '.' {
                expanded_map[coord.y() as usize][coord.x() as usize] = 'O';
                filled_positions.insert((coord.x() / 3, coord.y() / 3));
                for neighbor in coord.orthogonal_neighbors() {
                    if (0..expanded_map.len() as i32).contains(&neighbor.y())
                        && (0..expanded_map[0].len() as i32).contains(&neighbor.x())
                    {
                        q.push_back(neighbor);
                    }
                }
            }
        }
    }

    total_positions - filled_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(
        4,
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
    )]
    #[case(
        8,
        "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"
    )]
    fn test_func1(#[case] expected: i32, #[case] input: &str) {
        assert_eq!(expected, func1(input))
    }

    #[rstest]
    #[case(
        4,
        "...........
.S-------7.
.|F-----7|.
.||OOOOO||.
.||OOOOO||.
.|L-7OF-J|.
.|II|O|II|.
.L--JOL--J.
.....O....."
    )]
    #[case(
        4,
        "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
.........."
    )]
    #[case(
        8,
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
    )]
    #[case(
        10,
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
    )]
    fn test_func2(#[case] expected: usize, #[case] input: &str) {
        assert_eq!(expected, func2(input))
    }
}
