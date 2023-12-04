use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, space0, space1},
    multi::fold_many1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

use nom::Parser;
use rstest::rstest;
use std::{
    collections::{BTreeMap, HashSet},
    time::Instant,
};

struct Cards {
    my_cards: HashSet<u32>,
    winning_cards: HashSet<u32>,
}

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!(
        "First star: {}",
        input
            .lines()
            .map(|card| {
                match amount_of_winning_number(card).checked_sub(1) {
                    Some(num) => 2u32.pow(num),
                    None => 0,
                }
            })
            .sum::<u32>()
    );
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", total_cards(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn amount_of_winning_number(input: &str) -> u32 {
    let (_, cards) = parse_line(input).expect("Input format incorrect");
    cards.winning_cards.intersection(&cards.my_cards).count() as u32
}

fn set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<_>, item| {
            acc.insert(item);
            acc
        },
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, Cards> {
    // matches Card  1:
    let (input, _) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space0)),
    )(input)?;

    // parses the numbers into winning cards and my cards
    separated_pair(set, tuple((tag("|"), space1)), set)
        .map(|(winning_cards, my_cards)| Cards {
            winning_cards,
            my_cards,
        })
        .parse(input)
}

fn total_cards(input: &str) -> u32 {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| (i, amount_of_winning_number(line)))
        .fold(BTreeMap::new(), |mut acc, (card, num_of_winning_cards)| {
            let actual_amount = match acc.get(&card) {
                Some(amount) => *amount,
                None => {
                    acc.insert(card, 1);
                    1
                }
            };

            for other_card in (card + 1)..(card + 1 + num_of_winning_cards as usize) {
                let other_amount = match acc.get(&other_card) {
                    Some(amount) => *amount,
                    None => 1,
                };

                acc.insert(other_card, other_amount + actual_amount);
            }

            acc
        })
        .values()
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(8, "Card  1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")]
    #[case(2, "Card  2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19")]
    #[case(2, "Card  3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1")]
    #[case(1, "Card  4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83")]
    #[case(0, "Card  5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36")]
    #[case(0, "Card 64: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")]
    fn test_get_amount_of_winning_number(#[case] expected: i32, #[case] input: &str) {
        let amount = amount_of_winning_number(input);
        let points = if amount == 0 {
            0
        } else if amount == 1 {
            1
        } else {
            2u32.pow(amount - 1).try_into().unwrap()
        };
        assert_eq!(expected, points);
    }
    #[rstest]
    #[case(
        30,
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    )]
    fn test_get_total_cards(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, total_cards(input));
    }
}
