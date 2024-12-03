use itertools::Itertools;
use rstest::rstest;
use std::{cmp::Ordering, collections::BTreeMap, time::Instant};

fn parse_line(input: &str, altered: &bool) -> Bid {
    // input with format AQK34 234, first values are cards, second values are bid
    let mut it = input.split(' ');

    Bid {
        hand: Hand::new(
            it.next()
                .unwrap()
                .chars()
                .map(|card| Card::new(card, *altered))
                .collect(),
            altered,
        ),
        bid: it.last().unwrap().parse::<u32>().unwrap(),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (HandType::FiveOfAKind, HandType::FiveOfAKind)
            | (HandType::FourOfAKind, HandType::FourOfAKind)
            | (HandType::FullHouse, HandType::FullHouse)
            | (HandType::ThreeOfAKind, HandType::ThreeOfAKind)
            | (HandType::TwoPair, HandType::TwoPair)
            | (HandType::OnePair, HandType::OnePair)
            | (HandType::HighCard, HandType::HighCard) => std::cmp::Ordering::Equal,

            (HandType::FiveOfAKind, _) => std::cmp::Ordering::Greater,
            (_, HandType::FiveOfAKind) => std::cmp::Ordering::Less,

            (HandType::FourOfAKind, _) => std::cmp::Ordering::Greater,
            (_, HandType::FourOfAKind) => std::cmp::Ordering::Less,

            (HandType::FullHouse, _) => std::cmp::Ordering::Greater,
            (_, HandType::FullHouse) => std::cmp::Ordering::Less,

            (HandType::ThreeOfAKind, _) => std::cmp::Ordering::Greater,
            (_, HandType::ThreeOfAKind) => std::cmp::Ordering::Less,

            (HandType::TwoPair, _) => std::cmp::Ordering::Greater,
            (_, HandType::TwoPair) => std::cmp::Ordering::Less,

            (HandType::OnePair, _) => std::cmp::Ordering::Greater,
            (_, HandType::OnePair) => std::cmp::Ordering::Less,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd)]
struct Card {
    value: u8,
}

impl Card {
    fn new(value: char, altered: bool) -> Card {
        match value {
            '2' => Card { value: 2 },
            '3' => Card { value: 3 },
            '4' => Card { value: 4 },
            '5' => Card { value: 5 },
            '6' => Card { value: 6 },
            '7' => Card { value: 7 },
            '8' => Card { value: 8 },
            '9' => Card { value: 9 },
            'T' => Card { value: 10 },
            'J' => Card {
                value: if altered { 1 } else { 11 },
            },
            'Q' => Card { value: 12 },
            'K' => Card { value: 13 },
            'A' => Card { value: 14 },
            _ => panic!("Input error, Not a valid card"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: Vec<Card>, altered: &bool) -> Hand {
        let hand_type = compute_hand_type(&cards, altered);
        Hand { cards, hand_type }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            self.hand_type.cmp(&other.hand_type)
        } else {
            match self
                .cards
                .iter()
                .zip(other.cards.iter())
                .skip_while(|(this, that)| this.value == that.value)
                .next()
            {
                Some((this, that)) => this.value.cmp(&that.value),
                None => std::cmp::Ordering::Equal,
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Bid {
    hand: Hand,
    bid: u32,
}

impl Ord for Bid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for Bid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.hand.cmp(&other.hand))
    }
}

fn compute_hand_type(cards: &Vec<Card>, altered: &bool) -> HandType {
    let binding = cards.iter().counts_by(|card| card.value);

    if binding.len() == 1 {
        return HandType::FiveOfAKind;
    }

    let mut value_list: Vec<_> = binding.values().collect();

    value_list.sort();
    value_list.reverse();

    let first_value;
    let second_value;

    if !altered {
        first_value = **value_list.get(0).unwrap();
        second_value = **value_list.get(1).unwrap();
    } else {
        let (key, _) = binding.iter().max_by_key(|&(_, v)| v).unwrap();

        let jack_num = binding.get(&1).unwrap_or(&0);

        if key == &1 {
            let pre = **value_list.get(1).unwrap();
            first_value = pre + jack_num;
            second_value = match value_list.get(2) {
                Some(value) => **value,
                None => 0,
            };
        } else {
            let pre = **value_list.get(0).unwrap();
            first_value = pre + jack_num;
            second_value = match value_list.get(1) {
                Some(value) => **value,
                None => 0,
            };
        }
    }

    match first_value {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => match second_value {
            2 => HandType::FullHouse,
            1 => HandType::ThreeOfAKind,
            _ => panic!("Hand not well formed"),
        },
        2 => match second_value {
            2 => HandType::TwoPair,
            1 => HandType::OnePair,
            _ => panic!("Hand not well formed"),
        },
        1 => HandType::HighCard,
        num => {
            println!("{num}");
            panic!("Hand not well formed")
        }
    }
}

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", get_total_winnings(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn get_total_winnings(input: &str) -> u32 {
    let mut bids: Vec<Bid> = input.lines().map(|line| parse_line(line, &false)).collect();

    bids.sort();

    bids.iter()
        .enumerate()
        .map(|(i, bid)| bid.bid * (i as u32 + 1))
        .sum::<u32>()
}

fn func2(input: &str) -> u32 {
    let mut bids: Vec<Bid> = input.lines().map(|line| parse_line(line, &true)).collect();

    bids.sort();

    bids.iter()
        .enumerate()
        .map(|(i, bid)| bid.bid * (i as u32 + 1))
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(std::cmp::Ordering::Equal, HandType::FullHouse, HandType::FullHouse)] //1
    #[case(
        std::cmp::Ordering::Greater,
        HandType::FiveOfAKind,
        HandType::FourOfAKind
    )] //2
    #[case(
        std::cmp::Ordering::Greater,
        HandType::FourOfAKind,
        HandType::FullHouse
    )] //3
    #[case(
        std::cmp::Ordering::Greater,
        HandType::FullHouse,
        HandType::ThreeOfAKind
    )] //4
    #[case(std::cmp::Ordering::Greater, HandType::ThreeOfAKind, HandType::TwoPair)] //5
    #[case(std::cmp::Ordering::Greater, HandType::TwoPair, HandType::OnePair)] //6
    #[case(std::cmp::Ordering::Greater, HandType::OnePair, HandType::HighCard)] //7
    fn test_handtype_ordering(
        #[case] expected: std::cmp::Ordering,
        #[case] input1: HandType,
        #[case] input2: HandType,
    ) {
        assert_eq!(expected, input1.cmp(&input2));
    }

    #[rstest]
    #[case(std::cmp::Ordering::Equal, Hand::new(vec![
                Card {
                    value: 13,
                },
                Card {
                    value: 13,
                },
                Card {
                    value: 6,
                },
                Card {
                    value: 7,
                },
                Card {
                    value: 7,
                },
            ], &false), Hand::new(vec![
                Card {
                    value: 13,
                },
                Card {
                    value: 13,
                },
                Card {
                    value: 6,
                },
                Card {
                    value: 7,
                },
                Card {
                    value: 7,
                },
            ], &false

        ))] //1
    #[case(std::cmp::Ordering::Greater, Hand::new(vec![
                Card {
                    value: 13,
                },
                Card {
                    value: 13,
                },
                Card {
                    value: 6,
                },
                Card {
                    value: 7,
                },
                Card {
                    value: 7,
                },
            ], &false), Hand::new(vec![
                Card {
                    value: 13,
                },
                Card {
                    value: 13,
                },
                Card {
                    value: 5,
                },
                Card {
                    value: 7,
                },
                Card {
                    value: 7,
                },
            ], &false

        ))] //1
    #[case(std::cmp::Ordering::Greater, Hand::new(vec![
                Card {
                    value: 10,
                },
                Card {
                    value: 5,
                },
                Card {
                    value: 5,
                },
                Card {
                    value: 11,
                },
                Card {
                    value: 5,
                },
            ], &false), Hand::new(vec![
                Card {
                    value: 13,
                },
                Card {
                    value: 10,
                },
                Card {
                    value: 11,
                },
                Card {
                    value: 11,
                },
                Card {
                    value: 10,
                },
            ], &false

        ))] //1
    fn test_hand_ordering(
        #[case] expected: std::cmp::Ordering,
        #[case] input1: Hand,
        #[case] input2: Hand,
    ) {
        assert_eq!(expected, input1.cmp(&input2));
    }

    #[test]
    fn test_hand_bid() {
        let expected = std::cmp::Ordering::Greater;

        let hand1 = Hand::new(
            vec![
                Card { value: 10 },
                Card { value: 5 },
                Card { value: 5 },
                Card { value: 11 },
                Card { value: 5 },
            ],
            &false,
        );
        let hand2 = Hand::new(
            vec![
                Card { value: 13 },
                Card { value: 10 },
                Card { value: 11 },
                Card { value: 11 },
                Card { value: 10 },
            ],
            &false,
        );

        let bid1 = Bid {
            hand: hand1,
            bid: 1,
        };
        let bid2 = Bid {
            hand: hand2,
            bid: 2,
        };
        let mut bids = vec![bid1, bid2];

        bids.sort();

        dbg!(&bids);

        assert_eq!(expected, bids[1].cmp(&bids[0]));
    }

    #[test]
    fn test_get_total_winnings() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let winnings = 6440;

        assert_eq!(winnings, get_total_winnings(input));
    }

    #[rstest]
    #[case(HandType::FiveOfAKind, Hand::new(vec![
                Card {
                    value: 5,
                },
                Card {
                    value: 1,
                },
                Card {
                    value: 1,
                },
                Card {
                    value: 1,
                },
                Card {
                    value: 5,
                },
            ], &true)
        )] //1
    fn test_hand_type(#[case] expected: HandType, #[case] input1: Hand) {
        assert_eq!(expected, input1.hand_type);
    }

    #[rstest]
    #[case(std::cmp::Ordering::Equal, Hand::new(vec![
                Card {
                    value: 13,
                },
                Card {
                    value: 13,
                },
                Card {
                    value: 6,
                },
                Card {
                    value: 7,
                },
                Card {
                    value: 7,
                },
            ], &true), Hand::new(vec![
                Card {
                    value: 13,
                },
                Card {
                    value: 13,
                },
                Card {
                    value: 6,
                },
                Card {
                    value: 7,
                },
                Card {
                    value: 7,
                },
            ], &true

        ))] //1
    #[case(std::cmp::Ordering::Less, Hand::new(vec![
                Card {
                    value: 10,
                },
                Card {
                    value: 5,
                },
                Card {
                    value: 5,
                },
                Card {
                    value: 1,
                },
                Card {
                    value: 5,
                },
            ], &true), Hand::new(vec![
                Card {
                    value: 13,
                },
                Card {
                    value: 10,
                },
                Card {
                    value: 1,
                },
                Card {
                    value: 1,
                },
                Card {
                    value: 10,
                },
            ], &true

        ))] //1
    #[case(std::cmp::Ordering::Greater, Hand::new(vec![
                Card {
                    value: 5,
                },
                Card {
                    value: 1,
                },
                Card {
                    value: 1,
                },
                Card {
                    value: 1,
                },
                Card {
                    value: 5,
                },
            ], &true), Hand::new(vec![
                Card {
                    value: 2,
                },
                Card {
                    value: 10,
                },
                Card {
                    value: 1,
                },
                Card {
                    value: 1,
                },
                Card {
                    value: 10,
                },
            ], &true

        ))] //1
    fn test_hand_ordering_variation(
        #[case] expected: std::cmp::Ordering,
        #[case] input1: Hand,
        #[case] input2: Hand,
    ) {
        dbg!(&input1);
        assert_eq!(expected, input1.cmp(&input2));
    }

    #[test]
    fn test_get_total_winnings_variation() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let winnings = 5905;

        assert_eq!(winnings, func2(input));
    }
}
