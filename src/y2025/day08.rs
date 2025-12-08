use core::panic;
use std::{cmp::Reverse, collections::HashSet, time::Instant};

use glam::IVec3;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::{self, complete::newline},
    multi::separated_list1,
    IResult, Parser,
};

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input, 1000));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn parse(input: &str) -> IResult<&str, Vec<IVec3>> {
    separated_list1(
        newline,
        separated_list1(tag(","), character::complete::i32).map(|v| IVec3::new(v[0], v[1], v[2])),
    )(input)
}

fn join_circuits(mut circuits: Vec<HashSet<IVec3>>, a: &IVec3, b: &IVec3) -> Vec<HashSet<IVec3>> {
    let mut indices: Vec<usize> = circuits
        .iter()
        .positions(|circuit| circuit.contains(a) || circuit.contains(b))
        .collect();

    indices.sort_unstable_by(|a, b| b.cmp(a));

    let i1 = indices[0];
    let i2 = indices[1];

    let mut s1 = circuits.remove(i1);
    let s2 = circuits.remove(i2);

    s1.extend(s2);
    circuits.push(s1);
    circuits
}

fn add_connection(mut circuits: Vec<HashSet<IVec3>>, a: &IVec3, b: &IVec3) -> Vec<HashSet<IVec3>> {
    let added = circuits
        .iter_mut()
        .filter(|circuit| circuit.contains(a) || circuit.contains(b))
        .map(|circuit| {
            circuit.insert(*a);
            circuit.insert(*b);
        })
        .count();

    if added == 0 {
        circuits.push([*a, *b].iter().copied().collect());
    } else if added == 1 {
        // Nothing to do
    } else {
        // added > 1
        circuits = join_circuits(circuits, a, b);
    }

    circuits
}

#[tracing::instrument(skip(input))]
fn func1(input: &str, n: usize) -> usize {
    let (_, points) = parse(input).unwrap();
    let mut circuits: Vec<HashSet<IVec3>> = Vec::new();

    for (a, b, _) in points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, a.as_vec3().distance(b.as_vec3())))
        .sorted_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .take(n)
    {
        circuits = add_connection(circuits, a, b);
    }

    circuits.sort_by_key(|s| Reverse(s.len()));
    circuits.iter().take(3).fold(1, |acc, s| acc * s.len())
}

#[tracing::instrument(skip(input))]
fn func2(input: &str) -> i32 {
    let (_, points) = parse(input).unwrap();
    let mut circuits: Vec<HashSet<IVec3>> = Vec::new();
    let total_points = input.lines().count();

    for (a, b, _) in points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, a.as_vec3().distance(b.as_vec3())))
        .sorted_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
    {
        circuits = add_connection(circuits, a, b);

        if circuits.len() == 1 && circuits[0].len() == total_points {
            return a.x * b.x;
        }
    }
    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_func1() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
        let expected = 40;

        assert_eq!(func1(input, 10), expected);
    }

    #[test]
    fn test_func2() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
        let expected = 25272;

        assert_eq!(func2(input), expected);
    }
}
