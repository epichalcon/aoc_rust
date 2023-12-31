use rstest::rstest;
use std::time::Instant;

use crate::utils::coords::Coordinates;

// First star: 9918828
//          time:64.6883ms ->58.3831ms
// Second star: 692506533832
//          time:65.4287ms ->57.8711ms

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func2(input, 2));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input, 1000000));
    println!("\t time:{:?}", start_time.elapsed());
}

fn func2(input: &str, expansion: usize) -> usize {
    let empty_rows: Vec<usize> = input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            if line.find('#').is_none() {
                Some(y)
            } else {
                None
            }
        })
        .collect();

    let empty_cols: Vec<usize> = (0..input.lines().next().unwrap().len())
        .filter_map(|x| {
            if input
                .lines()
                .all(|line| line.chars().nth(x).unwrap() != '#')
            {
                Some(x)
            } else {
                None
            }
        })
        .collect();

    let galaxies: Vec<Coordinates<i32>> =
        input
            .lines()
            .enumerate()
            .fold(Vec::new(), |acc, (y, line)| {
                line.chars().enumerate().fold(acc, |mut acc, (x, ch)| {
                    if ch == '#' {
                        acc.push(Coordinates::new(x as i32, y as i32));
                    }
                    acc
                })
            });

    galaxies
        .iter()
        .enumerate()
        .map(|(i, coord1)| {
            galaxies[(i + 1)..]
                .iter()
                .map(|coord2| {
                    let mut distance = coord1.orthogonal_distance(*coord2) as usize;

                    for row_usize in empty_rows.iter() {
                        let row = (*row_usize).try_into().unwrap();
                        if (coord1.y() <= row && row <= coord2.y())
                            || (coord2.y() <= row && row <= coord1.y())
                        {
                            distance += expansion - 1;
                        }
                    }

                    for col_usize in empty_cols.iter() {
                        let col = (*col_usize).try_into().unwrap();
                        if (coord1.x() <= col && col <= coord2.x())
                            || (coord2.x() <= col && col <= coord1.x())
                        {
                            distance += expansion - 1;
                        }
                    }

                    distance
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[rstest]
    #[case(
        12,
        "..........
..........
..........
..........
..........
.#........
..........
..........
..........
....#....."
    )]
    fn test_func1_cases(#[case] expected: usize, #[case] input: &str) {
        assert_eq!(expected, func2(input, 2));
    }

    #[test]
    fn test_func1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let result = 374;
        assert_eq!(result, func2(input, 2));
    }

    #[rstest]
    #[case(
        1030,
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        10
    )]
    #[case(
        8410,
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        100
    )]
    fn test_func2_cases(#[case] expected: usize, #[case] input: &str, #[case] expansion: usize) {
        assert_eq!(expected, func2(input, expansion));
    }

    #[test]
    fn test_real() {
        let input = utils::io::read(2023, 11);
        assert_eq!(9918828, func2(&input, 2));
        assert_eq!(692506533832, func2(&input, 1000000));
    }
}
