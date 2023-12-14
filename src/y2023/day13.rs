use rstest::rstest;
use std::{cmp::min, time::Instant};

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!(
        "First star: {}",
        input.split("\n\n").map(func1).sum::<usize>()
    );
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!(
        "Second star: {}",
        input.split("\n\n").map(func2).sum::<usize>()
    );
    println!("\t time:{:?}", start_time.elapsed());
}

fn func1(input: &str) -> usize {
    detect_horizontal_fold(input, 100)
        .or(detect_vertical_fold(input, 1))
        .unwrap_or(0)
}

fn detect_horizontal_fold(input: &str, multiplier: usize) -> Option<usize> {
    let mut mirror: Vec<&str> = vec![];

    for (i, line) in input.lines().enumerate() {
        if &line == mirror.last().unwrap_or(&"")
            && is_mirror(&input.lines().collect::<Vec<&str>>()[i..].to_vec(), &mirror)
        {
            return Some(i * multiplier);
        }
        mirror.push(line);
    }

    None
}

fn transpose_string(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let max_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let transposed: Vec<String> = (0..max_len)
        .map(|col| {
            lines
                .iter()
                .filter_map(|line| line.chars().nth(col))
                .collect::<String>()
        })
        .collect();

    transposed.join("\n")
}

fn detect_vertical_fold(input: &str, multiplier: usize) -> Option<usize> {
    let transpose_string = &transpose_string(input);
    let transposed = transpose_string.as_str();
    detect_horizontal_fold(transposed, multiplier)
}

fn is_mirror(input: &Vec<&str>, mirror: &Vec<&str>) -> bool {
    let mut reversed = mirror.clone();
    reversed.reverse();

    let min_len = min(reversed.len(), input.len());

    reversed[..min_len] == input[..min_len]
}

fn func2(input: &str) -> usize {
    detect_horizontal_fold_smudge(input, 100)
        .or(detect_vertical_fold_smudge(input, 1))
        .unwrap_or(0)
}

fn detect_horizontal_fold_smudge(input: &str, multiplier: usize) -> Option<usize> {
    let mut mirror: Vec<&str> = vec![];

    for (i, line) in input.lines().enumerate() {
        let differences = line_differences(&line, mirror.last().unwrap_or(&""));
        if differences <= 1
            && is_smudged_mirror(&input.lines().collect::<Vec<&str>>()[i..].to_vec(), &mirror)
        {
            return Some(i * multiplier);
        }
        mirror.push(line);
    }

    None
}

fn detect_vertical_fold_smudge(input: &str, multiplier: usize) -> Option<usize> {
    let transpose_string = &transpose_string(input);
    let transposed = transpose_string.as_str();
    detect_horizontal_fold_smudge(transposed, multiplier)
}

fn is_smudged_mirror(input: &Vec<&str>, mirror: &Vec<&str>) -> bool {
    let mut reversed = mirror.clone();
    reversed.reverse();

    let min_len = min(reversed.len(), input.len());

    reversed[..min_len]
        .iter()
        .zip(input[..min_len].iter())
        .map(|(line1, line2)| line_differences(line1, line2))
        .sum::<usize>()
        == 1
}

fn line_differences(line1: &str, line2: &str) -> usize {
    if line2 == "" {
        return 50;
    }
    line1
        .chars()
        .zip(line2.chars())
        .filter(|(ch1, ch2)| ch1 != ch2)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(
        500,
        "#.....
.#....
..#...
...#..
######
######"
    )]
    #[case(
        5,
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
    )]
    #[case(
        400,
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    )]
    #[case(
        200,
        "..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    )]
    #[case(
        3,
        "##..###
#.##.#.
......#
......#
#.##.#.
##..##.
#.##.#."
    )]
    #[case(
        6,
        ".#.##..##.#
..##.##.##.
##..#..#..#
.#..####..#
#..........
..##....##.
..##....##.
##..####..#
.##########
#....##....
..##.##.##.
#..........
#####..####"
    )]
    fn test_func1(#[case] expected: usize, #[case] input: &str) {
        assert_eq!(expected, func1(input))
    }

    #[test]
    fn test_func1_complete() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let expected = 405;

        assert_eq!(expected, input.split("\n\n").map(func1).sum::<usize>());
    }

    #[rstest]
    #[case(
        300,
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
    )]
    #[case(
        100,
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    )]
    #[case(
        1100,
        "#..###..#####
###..#.......
.##.#.#.###..
#####....####
#####....####
.##.#.#.###..
###..#.......
#..###..#####
#.##.##.....#
#########.###
#.##.#..#.###
#.##.#..#.###
#########.###
#.##.##.....#
#.####..#####
###..#.......
.##.#.#.###.."
    )]
    #[case(
        9,
        ".#.##..##.#
..##.##.##.
##..#..#..#
.#..####..#
#..........
..##....##.
..##....##.
##..####..#
.##########
#....##....
..##.##.##.
#..........
#####..####"
    )]
    #[case(
        200,
        ".##.
....
...#"
    )]
    fn test_func2(#[case] expected: usize, #[case] input: &str) {
        assert_eq!(expected, func2(input))
    }

    #[test]
    fn test_func2_complete() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let expected = 400;

        assert_eq!(expected, input.split("\n\n").map(func2).sum::<usize>());
    }
}
