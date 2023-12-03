use rstest::rstest;
use std::{cmp, time::Instant};

const MAX_RED: u8 = 12;
const MAX_GREEN: u8 = 13;
const MAX_BLUE: u8 = 14;

struct ColorSet {
    blues: Option<u32>,
    reds: Option<u32>,
    greens: Option<u32>,
}

impl ColorSet {
    fn max(&self, other: &ColorSet) -> ColorSet {
        ColorSet {
            blues: cmp::max(self.blues, other.blues),
            reds: cmp::max(self.reds, other.reds),
            greens: cmp::max(self.greens, other.greens),
        }
    }
}

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!(
        "Day 2 First star: {}",
        input.lines().filter_map(game_is_possible).sum::<u32>()
    );
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!(
        "Day 2 Second star: {}",
        input.lines().map(fewest_color_cubes).sum::<u32>()
    );
    println!("\t time:{:?}", start_time.elapsed());
}

fn game_is_possible(input: &str) -> Option<u32> {
    let mut game_iterator = input.split(": ");

    let game_number = game_iterator
        .next()
        .unwrap()
        .replace("Game ", "")
        .parse::<u32>()
        .unwrap();

    let mut cube_set = game_iterator
        .next()
        .expect("Incorrect input format")
        .split("; ");

    let correct = cube_set.all(|set| {
        set.split(", ").into_iter().all(|num_color| {
            let mut num_color_it = num_color.split(" ");

            let number = num_color_it
                .next()
                .unwrap()
                .parse::<u8>()
                .expect("should be a number");

            let color = num_color_it.next().unwrap();

            match color {
                "blue" => number <= MAX_BLUE,
                "red" => number <= MAX_RED,
                "green" => number <= MAX_GREEN,
                _ => panic!("Not a valid color"),
            }
        })
    });

    if correct {
        Some(game_number)
    } else {
        None
    }
}
fn fewest_color_cubes(input: &str) -> u32 {
    let cube_set = input
        .split(": ")
        .last()
        .expect("Incorrect input format")
        .split("; ");

    let max_colored_cubes = cube_set.fold(
        ColorSet {
            blues: None,
            reds: None,
            greens: None,
        },
        |max_set, set| {
            let color_set = set.split(", ").into_iter().fold(
                ColorSet {
                    blues: None,
                    reds: None,
                    greens: None,
                },
                |mut set, num_color| {
                    let mut num_color_it = num_color.split(" ");

                    let number = num_color_it
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .expect("should be a number");

                    let color = num_color_it.next().unwrap();

                    match color {
                        "blue" => set.blues = Some(number),
                        "red" => set.reds = Some(number),
                        "green" => set.greens = Some(number),
                        _ => panic!("Not a valid color"),
                    }

                    set
                },
            );
            max_set.max(&color_set)
        },
    );

    max_colored_cubes.greens.unwrap_or(1)
        * max_colored_cubes.reds.unwrap_or(1)
        * max_colored_cubes.blues.unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(Some(1), "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")]
    #[case(
        Some(2),
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
    )]
    #[case(
        None,
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
    )]
    #[case(
        None,
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
    )]
    #[case(
        Some(4),
        "Game 4: 13 green, 3 red, 6 blue; 3 green, 12 red; 3 green, 14 blue, 1 red"
    )]
    #[case(Some(5), "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")]
    fn test_game_is_posible_individual(#[case] expected: Option<u32>, #[case] test_case: &str) {
        assert_eq!(expected, game_is_possible(test_case))
    }

    #[test]
    fn test_game_is_possible_global() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, input.lines().filter_map(game_is_possible).sum::<u32>());
    }

    #[rstest]
    #[case(48, "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")]
    #[case(12, "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")]
    #[case(
        1560,
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
    )]
    #[case(
        630,
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
    )]
    #[case(36, "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")]
    fn test_game_fuest_color_individual(#[case] expected: u32, #[case] test_case: &str) {
        assert_eq!(expected, fewest_color_cubes(test_case))
    }
}

// con regex funciona pero es
// fn game_is_possible(input: &str) -> Option<u32> {
//     let mut game_iterator = input.split(": ");
//
//     let game_number = game_iterator
//         .next()
//         .unwrap()
//         .replace("Game ", "")
//         .parse::<u32>()
//         .unwrap();
//
//     let pattern = regex::Regex::new(", |; ").unwrap();
//
//     let correct = pattern
//         .split(game_iterator.next().expect("incorrect input format"))
//         .all(|num_color| {
//             let mut num_color_it = num_color.split(" ");
//
//             let number = num_color_it
//                 .next()
//                 .unwrap()
//                 .parse::<u8>()
//                 .expect("should be a number");
//
//             let color = num_color_it.next().unwrap();
//
//             match color {
//                 "blue" => number <= MAX_BLUE,
//                 "red" => number <= MAX_RED,
//                 "green" => number <= MAX_GREEN,
//                 _ => panic!("Not a valid color"),
//             }
//         });
//
//     if correct {
//         Some(game_number)
//     } else {
//         None
//     }
// }
