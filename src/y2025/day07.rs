use std::{collections::HashSet, time::Instant};

use nom::InputIter;

use crate::utils::{coords::Coordinates, multiset::MultiSet};

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn parse(input: &str) -> (Coordinates<usize>, HashSet<Coordinates<usize>>) {
    let splitters: HashSet<Coordinates<usize>> =
        input
            .lines()
            .enumerate()
            .fold(HashSet::new(), |acc, (y, row)| {
                let n_splitters = row
                    .chars()
                    .enumerate()
                    .filter_map(|(x, c)| match c {
                        '^' => Some(Coordinates::new(x, y)),
                        _ => None,
                    })
                    .collect::<HashSet<Coordinates<usize>>>();
                acc.union(&n_splitters).cloned().collect()
            });

    let sx = input.position(|c| c == 'S').unwrap();

    (Coordinates::new(sx, 0), splitters)
}

fn progress_beams(beams: &HashSet<Coordinates<usize>>) -> HashSet<Coordinates<usize>> {
    beams.iter().filter_map(|beam| beam.try_up()).collect()
}

fn split_beams(
    splitters: &HashSet<Coordinates<usize>>,
    beams: &HashSet<Coordinates<usize>>,
) -> (usize, HashSet<Coordinates<usize>>) {
    let beams_to_split: HashSet<_> = splitters & beams;

    let splitted_beams: HashSet<Coordinates<usize>> =
        beams_to_split.iter().fold(HashSet::new(), |mut acc, beam| {
            acc.extend(beam.try_left());
            acc.extend(beam.try_right());
            acc
        });

    let new_beams: HashSet<_> = &(beams - &beams_to_split) | &splitted_beams;

    (beams_to_split.len(), new_beams)
}

#[tracing::instrument(skip(input))]
fn func1(input: &str) -> usize {
    let (initial, splitters) = parse(input);

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut rep = initial_graphic_representation(width, height, &initial, &splitters);
    let mut beams = HashSet::new();
    beams.insert(initial);
    println!("{}", rep);

    let mut i = 0;
    let mut res = 0;

    while i < height {
        let (n_splitted, new_beams) = split_beams(&splitters, &progress_beams(&beams));
        res += n_splitted;

        let n_rep = step_graphic_representation(&rep, &new_beams);
        println!("{}", rep);

        beams = new_beams;
        rep = n_rep;

        i += 1;
    }

    res
}

fn progress_quantum_beams(beams: &MultiSet<Coordinates<usize>>) -> MultiSet<Coordinates<usize>> {
    beams
        .into_iter()
        .filter_map(|(coord, times)| coord.try_up().map(|new_coord| (new_coord, *times)))
        .collect()
}

fn split_quantum_beams(
    splitters: &HashSet<Coordinates<usize>>,
    beams: &MultiSet<Coordinates<usize>>,
) -> MultiSet<Coordinates<usize>> {
    let beams_to_split: MultiSet<Coordinates<usize>> = beams
        .iter()
        .filter(|(elem, _)| splitters.contains(elem))
        .map(|(&elem, count)| (elem, count))
        .collect();

    let splitted_beams: MultiSet<Coordinates<usize>> =
        beams_to_split
            .iter()
            .fold(MultiSet::new(), |mut acc, (coord, times)| {
                acc.extend(coord.try_left(), times);
                acc.extend(coord.try_right(), times);
                acc
            });

    let new_beams: MultiSet<_> = (beams - &beams_to_split) | &splitted_beams;
    new_beams
}

#[tracing::instrument(skip(input))]
fn func2(input: &str) -> usize {
    let (initial, splitters) = parse(input);

    let height = input.lines().count();
    let mut beams = MultiSet::new();
    beams.insert(initial, 1);

    let mut i = 0;

    while i < height {
        let new_beams = split_quantum_beams(&splitters, &progress_quantum_beams(&beams));
        beams = new_beams;
        i += 1;
    }

    beams.len()
}

fn initial_graphic_representation(
    width: usize,
    height: usize,
    start: &Coordinates<usize>,
    splitters: &HashSet<Coordinates<usize>>,
) -> String {
    let mut s = String::new();
    for y in 0..height {
        for x in 0..width {
            let coord = Coordinates::new(x, y);

            if coord == *start {
                s.push('S');
            } else if splitters.contains(&coord) {
                s.push('^');
            } else {
                s.push(' ');
            }
        }
        s.push('\n');
    }
    s
}

fn step_graphic_representation(
    pre_grephic_repr: &str,
    beams: &HashSet<Coordinates<usize>>,
) -> String {
    pre_grephic_repr
        .lines()
        .enumerate()
        .map(|(y, row)| {
            let mut s = row
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == ' ' && beams.contains(&Coordinates::new(x, y)) {
                        '|'
                    } else {
                        c
                    }
                })
                .collect::<String>();
            s.push('\n');
            s
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_func1() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let expected = 21;
        assert_eq!(func1(input), expected);
    }

    #[test]
    fn test_func2() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let expected = 40;
        assert_eq!(func2(input), expected);
    }
}
