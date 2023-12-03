use rstest::rstest;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use crate::utils::coords::Coordinates;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct PartNumber {
    value: u32,
    positions: Vec<Coordinates<u32>>,
}
impl PartNumber {
    fn new() -> PartNumber {
        PartNumber {
            value: 0,
            positions: vec![],
        }
    }

    fn get_near_symbols(&self, symbol_positions: &HashSet<Coordinates<u32>>) -> Option<u32> {
        // any neighbor intersects with a symbol_position
        if self.positions.clone().into_iter().any(|coord| {
            coord
                .all_neighbors()
                .intersection(symbol_positions)
                .next() // if intersection len > 0 -> Some() else None
                .is_some()
        }) {
            Some(self.value)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Gear {
    position: Coordinates<u32>,
    part1: PartNumber,
    part2: PartNumber,
}

impl Gear {
    fn new(x: u32, y: u32) -> Gear {
        Gear {
            position: Coordinates::new(x, y),
            part1: PartNumber::new(),
            part2: PartNumber::new(),
        }
    }

    fn get_parts_near<'a>(
        &'a self,
        coord_to_part: &'a HashMap<Coordinates<u32>, PartNumber>,
    ) -> HashSet<PartNumber> {
        self.position
            .all_neighbors()
            .into_iter()
            .fold(HashSet::new(), |mut acc, position| {
                match coord_to_part.get(&position.clone()) {
                    Some(part) => acc.insert(part.clone()),
                    None => false,
                };

                acc
            })
    }

    fn gear_ratio(&self, coord_to_part: &HashMap<Coordinates<u32>, PartNumber>) -> Option<u32> {
        let near_parts = self.get_parts_near(coord_to_part);
        if near_parts.len() == 2 {
            Some(near_parts.into_iter().map(|part| part.value).product())
        } else {
            None
        }
    }
}

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("Day 3 first star: {}", get_missing_part(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Day 3 second star: {}", gear_ratios(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn get_missing_part(input: &str) -> u32 {
    let mut part_numbers: Vec<PartNumber> = vec![];
    let mut symbol_positions: HashSet<Coordinates<u32>> = HashSet::new();
    let mut current_part_number = PartNumber::new();
    let mut in_number: bool = false;

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                current_part_number.value =
                    current_part_number.value * 10 + ch.to_digit(10).unwrap();

                current_part_number.positions.push(Coordinates::new(
                    x.try_into().unwrap(),
                    y.try_into().unwrap(),
                ));

                in_number = true;
            } else {
                if in_number {
                    part_numbers.push(current_part_number);

                    current_part_number = PartNumber::new();

                    in_number = false;
                }
                if ch != '.' {
                    symbol_positions.insert(Coordinates::new(
                        x.try_into().unwrap(),
                        y.try_into().unwrap(),
                    ));
                }
            }
        }
    }
    part_numbers.push(current_part_number);

    part_numbers
        .into_iter()
        .filter_map(|position| position.get_near_symbols(&symbol_positions))
        .sum::<u32>()
}

fn gear_ratios(input: &str) -> u32 {
    let mut part_numbers: Vec<PartNumber> = vec![];
    let mut gears: Vec<Gear> = vec![];
    let mut current_part_number = PartNumber::new();
    let mut in_number: bool = false;
    let mut coord_to_part = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                current_part_number.value =
                    current_part_number.value * 10 + ch.to_digit(10).unwrap();

                current_part_number.positions.push(Coordinates::new(
                    x.try_into().unwrap(),
                    y.try_into().unwrap(),
                ));

                in_number = true;
            } else {
                if in_number {
                    for coord in &current_part_number.positions.clone() {
                        coord_to_part.insert(*coord, current_part_number.clone());
                    }

                    part_numbers.push(current_part_number);

                    current_part_number = PartNumber::new();

                    in_number = false;
                }
                if ch == '*' {
                    gears.push(Gear::new(x.try_into().unwrap(), y.try_into().unwrap()));
                }
            }
        }
    }
    for coord in &current_part_number.positions {
        coord_to_part.insert(*coord, current_part_number.clone());
    }
    part_numbers.push(current_part_number);

    gears
        .into_iter()
        .filter_map(|gear| gear.gear_ratio(&coord_to_part))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(467, ".467*")]
    #[case(0, ".467.")]
    #[case(0, "467")]
    #[case(468, ".467*1")]
    #[case(
        467,
        "467..114..
...*......"
    )]
    #[case(
        4361,
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
    )]
    fn test_get_spelled_coords_multiple_numbers(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, get_missing_part(input));
    }

    #[rstest]
    #[case(0, ".467*")]
    #[case(0, ".467.")]
    #[case(0, "467")]
    #[case(467, ".467*1")]
    #[case(
        0,
        "467..114..
...*......"
    )]
    #[case(
        467835,
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
    )]
    fn test_get_gear_power(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, gear_ratios(input));
    }
}
