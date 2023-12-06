use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, digit1, newline, space0},
    combinator::map_res,
    multi::{fold_many1, many_till, separated_list1},
    sequence::{separated_pair, terminated, tuple},
    IResult, Parser,
};
use rstest::rstest;
use std::{i128, ops::Index, time::Instant, u128};

#[derive(Debug, Clone)]
struct Mapping {
    destination_range_start: u128,
    source_range_start: u128,
    range_length: u128,
}

impl Mapping {
    fn contains(&self, seed_range: SeedRange) -> bool {
        self.source_range_start <= seed_range.initial_seed
            && seed_range.initial_seed < self.source_range_start + self.range_length
            || self.source_range_start <= seed_range.final_seed
                && seed_range.final_seed < self.source_range_start + self.range_length
    }

    fn translate_range(&self, seed_range: SeedRange) -> SeedRange {
        let offset_start = seed_range.initial_seed as i128 - self.source_range_start as i128;
        let offset_end = seed_range.final_seed as i128 - self.source_range_start as i128;

        SeedRange {
            initial_seed: (self.destination_range_start as i128 + offset_start) as u128,
            final_seed: (self.destination_range_start as i128 + offset_end) as u128,
        }
    }
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u128>,
    mappings: Vec<Vec<Mapping>>,
}

#[derive(Debug, Clone)]
struct SeedRange {
    initial_seed: u128,
    final_seed: u128,
}

impl SeedRange {
    fn new(initial_seed: u128, final_seed: u128) -> SeedRange {
        SeedRange {
            initial_seed,
            final_seed,
        }
    }

    fn default() -> SeedRange {
        SeedRange::new(0, 0)
    }
}

fn parse_input(input: &str) -> IResult<&str, Almanac> {
    separated_pair(seeds, tuple((newline, newline)), transformations)
        .map(|(seed_list, mapping_list)| Almanac {
            seeds: seed_list,
            mappings: mapping_list,
        })
        .parse(input)
}

fn seeds(input: &str) -> IResult<&str, Vec<u128>> {
    let (input, _) = tag("seeds: ")(input)?;
    fold_many1(
        terminated(complete::u128, space0),
        Vec::new,
        |mut acc, item| {
            acc.push(item);
            acc
        },
    )(input)
}

fn transformations(input: &str) -> IResult<&str, Vec<Vec<Mapping>>> {
    fold_many1(terminated(mappings, newline), Vec::new, |mut acc, item| {
        acc.push(item);
        acc
    })(input)
}

fn mappings(input: &str) -> IResult<&str, Vec<Mapping>> {
    // matches Card  1:
    let (input, _) = many_till(take_until("\n"), newline)(input)?;

    fold_many1(terminated(mapping, newline), Vec::new, |mut acc, item| {
        acc.push(item);
        acc
    })(input)
}

fn mapping(input: &str) -> IResult<&str, Mapping> {
    separated_list1(tag(" "), parse_u128)
        .map(|inputs| Mapping {
            destination_range_start: *inputs.index(0),
            source_range_start: *inputs.index(1),
            range_length: *inputs.index(2),
        })
        .parse(input)
}

fn parse_u128(input: &str) -> IResult<&str, u128> {
    map_res(digit1, |s: &str| s.parse::<u128>())(input)
}

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("Day 5 First star: {}", find_seed_destinations(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", get_minimum_seed_range(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn find_seed_destinations(input: &str) -> u128 {
    let (_, parsed) = parse_input(input).expect("Incorrect format");

    let mut transformation = parsed.seeds;

    for mapping in &parsed.mappings {
        transformation = transformation
            .into_iter()
            .map(|source| map_source_to_destination(source, &mapping))
            .collect();
    }

    transformation.into_iter().min().unwrap()
}

fn apply_mapping(source: u128, mappings: &Mapping) -> Option<u128> {
    if mappings.source_range_start <= source
        && source <= mappings.source_range_start + mappings.range_length
    {
        let detination = source - mappings.source_range_start + mappings.destination_range_start;
        Some(detination)
    } else {
        None
    }
}

fn map_source_to_destination(source: u128, mappings: &Vec<Mapping>) -> u128 {
    match mappings
        .into_iter()
        .skip_while(|mapping| apply_mapping(source, mapping).is_none())
        .next()
        .map(|mapping| apply_mapping(source, mapping))
        .unwrap_or(None)
    {
        Some(num) => num,
        None => source,
    }
}

fn get_minimum_seed_range(input: &str) -> u128 {
    let (_, parsed) = parse_input(input).expect("Incorrect format");

    let ranges: Vec<SeedRange> = parsed
        .seeds
        .chunks(2)
        .map(|chunk| {
            let initial_seed = *chunk.first().unwrap();
            let final_seed = initial_seed + chunk.last().unwrap() - 1;
            SeedRange::new(initial_seed, final_seed)
        })
        .collect();

    map_seeds_to_location(ranges, parsed.mappings)
        .into_iter()
        .map(|range| range.initial_seed)
        .min()
        .unwrap()
}

fn map_seeds_to_location(
    seed_ranges: Vec<SeedRange>,
    mapping_per_translation: Vec<Vec<Mapping>>,
) -> Vec<SeedRange> {
    let mut mutable_seed_ranges = seed_ranges.clone();

    for segment_maps in mapping_per_translation {
        let mut next_seed_ranges: Vec<SeedRange> = vec![];
        let mut i = 0;

        while i < mutable_seed_ranges.len() {
            let mut current_range = mutable_seed_ranges[i].clone();
            let mut outside_ranges;

            let mut mapped = false;
            for segment in &segment_maps {
                if segment.contains(current_range.clone()) {
                    mapped = true;
                    (current_range, outside_ranges) =
                        trim_seed_ranges(current_range.clone(), segment.clone());

                    let translation = segment.translate_range(current_range.clone());

                    next_seed_ranges.push(translation);
                    mutable_seed_ranges.append(&mut outside_ranges);
                    break;
                }
            }
            if !mapped {
                next_seed_ranges.push(current_range.clone());
            }
            i += 1;
        }
        mutable_seed_ranges = next_seed_ranges;
    }

    mutable_seed_ranges
}

fn trim_seed_ranges(seed_range: SeedRange, mapping: Mapping) -> (SeedRange, Vec<SeedRange>) {
    let mut outside_ranges: Vec<SeedRange> = vec![];
    let mut new_seed_range = seed_range.clone();

    let segment_start = mapping.source_range_start;
    let segment_end = mapping.source_range_start + mapping.range_length - 1;

    if seed_range.initial_seed < segment_start {
        outside_ranges.push(SeedRange::new(seed_range.initial_seed, segment_start - 1));
        new_seed_range.initial_seed = segment_start;
    }

    if seed_range.final_seed > segment_end {
        outside_ranges.push(SeedRange::new(segment_end + 1, seed_range.final_seed));
        new_seed_range.final_seed = segment_end;
    }

    (new_seed_range, outside_ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_mapping() {
        let mapping = Mapping {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };

        assert_eq!(Some(51), apply_mapping(99, &mapping))
    }

    #[rstest]
    #[case(0, 0)] //1
    #[case(1, 1)] //2
    #[case(48, 48)] //3
    #[case(49, 49)] //4
    #[case(50, 52)] //5
    #[case(51, 53)] //6
    #[case(96, 98)] //7
    #[case(97, 99)] //8
    #[case(98, 50)] //9
    #[case(99, 51)] //10
    fn test_source_to_destination(#[case] seed: u128, #[case] soil: u128) {
        let mapping = vec![
            Mapping {
                destination_range_start: 50,
                source_range_start: 98,
                range_length: 2,
            },
            Mapping {
                destination_range_start: 52,
                source_range_start: 50,
                range_length: 48,
            },
        ];

        assert_eq!(soil, map_source_to_destination(seed, &mapping));
    }

    // #[rstest]
    // #[case()]
    // fn test_func2(#[case] expected: i32, #[case] input: &str) {}

    #[test]
    fn test_find_location() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        dbg!(input);

        let mapping = Mapping {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };

        assert_eq!(35, find_seed_destinations(input))
    }

    #[test]
    fn test_find_location_ranges() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        dbg!(input);

        let mapping = Mapping {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };

        assert_eq!(46, get_minimum_seed_range(input))
    }
}
