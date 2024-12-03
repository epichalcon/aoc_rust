use itertools::Intersperse;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alpha0, anychar, digit0, digit1, newline},
    multi::{fold_many1, many1, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};
use rstest::rstest;
use std::{
    collections::{BTreeMap, VecDeque},
    time::Instant,
};

use crate::utils::parser::get_number_from_line;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", calculate_accepted_pieces(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", calculate_all_accepted_pieces(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Piece>, BTreeMap<String, Workflow>)> {
    let (input, (workflows, seeds)) =
        separated_pair(parse_workflows, tuple((newline, newline)), parse_piece)(input)?;

    Ok((input, (seeds, workflows)))
}

fn parse_piece(input: &str) -> IResult<&str, Vec<Piece>> {
    many1(delimited(
        tag("{"),
        piece_contents,
        tuple((tag("}"), newline)),
    ))(input)
}

fn parse_individual_rating(input: &str) -> IResult<&str, u128> {
    let (input, _rating) = anychar(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, x) = digit0(input)?;

    Ok((input, x.parse().unwrap()))
}

fn piece_contents(input: &str) -> IResult<&str, Piece> {
    let (input, nums) = separated_list0(tag(","), parse_individual_rating)(input)?;

    let mut num_iter = nums.iter();

    Ok((
        input,
        Piece {
            x: *num_iter.next().unwrap(),
            m: *num_iter.next().unwrap(),
            a: *num_iter.next().unwrap(),
            s: *num_iter.next().unwrap(),
        },
    ))
}

fn parse_workflows(input: &str) -> IResult<&str, BTreeMap<String, Workflow>> {
    fold_many1(
        terminated(parse_workflow, newline),
        BTreeMap::new,
        |mut acc: BTreeMap<String, Workflow>, (name, wf)| {
            acc.insert(name, wf);
            acc
        },
    )(input)
}

fn parse_workflow(input: &str) -> IResult<&str, (String, Workflow)> {
    let (input, name) = alpha0(input)?;
    let (input, result) =
        delimited(tag("{"), separated_list0(tag(","), parse_rule), tag("}"))(input)?;
    Ok((
        input,
        (
            name.to_string(),
            Workflow {
                rules: result[0..result.len() - 1].to_vec(),
                default: result[result.len() - 1].destination.clone(),
            },
        ),
    ))
}

fn parse_rule(input: &str) -> IResult<&str, Rules> {
    if input.chars().nth(1).unwrap() == '>' || input.chars().nth(1).unwrap() == '<' {
        let (input, rating) = anychar(input)?;
        let (input, operator) = anychar(input)?;
        let (input, quantity) = digit1(input)?;
        let (input, destination) = preceded(tag(":"), alpha0)(input)?;
        Ok((
            input,
            Rules {
                rating: rating.to_string(),
                operator,
                quantity: quantity.parse().unwrap(),
                destination: match destination {
                    "A" => Destination::Accepted,
                    "R" => Destination::Regected,
                    other => Destination::Workflow(other.to_string()),
                },
            },
        ))
    } else {
        let (input, destination) = alpha0(input)?;
        Ok((
            input,
            Rules {
                rating: "".to_string(),
                operator: ' ',
                quantity: 0,
                destination: match destination {
                    "A" => Destination::Accepted,
                    "R" => Destination::Regected,
                    other => Destination::Workflow(other.to_string()),
                },
            },
        ))
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rules>,
    default: Destination,
}

impl Workflow {
    fn get_destination(&self, piece: &Piece) -> Destination {
        let mut iter_rules = self.rules.iter();
        loop {
            match iter_rules.next() {
                Some(rule) => match rule.get_destination(piece) {
                    Some(destination) => return destination,
                    None => (),
                },
                None => return self.default.clone(),
            }
        }
    }

    fn calculate_new_intervals(
        &self,
        x: (u128, u128),
        m: (u128, u128),
        a: (u128, u128),
        s: (u128, u128),
    ) -> (
        VecDeque<(
            String,
            (u128, u128),
            (u128, u128),
            (u128, u128),
            (u128, u128),
        )>,
        u128,
    ) {
        let mut new_intervals = VecDeque::new();
        let mut accepted_total = 0;
        let mut new_x = x;
        let mut new_m = m;
        let mut new_a = a;
        let mut new_s = s;
        let mut accepted = 0;
        let mut new_interval = None;

        for rule in &self.rules {
            (new_interval, new_x, new_m, new_a, new_s, accepted) =
                rule.calculate_new_interval(new_x, new_m, new_a, new_s);
            accepted_total += accepted;
            match new_interval {
                Some(interval) => new_intervals.push_back(interval),
                None => (),
            }
        }

        match &self.default {
            Destination::Workflow(other) => {
                new_intervals.push_back((other.to_string(), new_x, new_m, new_a, new_s))
            }
            Destination::Accepted => {
                let total_x = new_x.1 - new_x.0 + 1;
                let total_m = new_m.1 - new_m.0 + 1;
                let total_a = new_a.1 - new_a.0 + 1;
                let total_s = new_s.1 - new_s.0 + 1;
                accepted_total += total_s * total_m * total_x * total_a
            }
            Destination::Regected => (),
        }

        (new_intervals, accepted_total)
    }
}

#[derive(Debug, Clone)]
struct Rules {
    rating: String,
    operator: char,
    quantity: u128,
    destination: Destination,
}

impl Rules {
    fn get_destination(&self, piece: &Piece) -> Option<Destination> {
        match self.rating.as_str() {
            "x" => {
                if self.follows_rule(piece.x) {
                    return Some(self.destination.clone());
                } else {
                    return None;
                }
            }
            "m" => {
                if self.follows_rule(piece.m) {
                    return Some(self.destination.clone());
                } else {
                    return None;
                }
            }
            "a" => {
                if self.follows_rule(piece.a) {
                    return Some(self.destination.clone());
                } else {
                    return None;
                }
            }
            "s" => {
                if self.follows_rule(piece.s) {
                    return Some(self.destination.clone());
                } else {
                    return None;
                }
            }

            _ => panic!(),
        }
    }

    fn follows_rule(&self, rating: u128) -> bool {
        match self.operator {
            '<' => rating < self.quantity,
            '>' => rating > self.quantity,
            _ => panic!(),
        }
    }

    fn calculate_new_interval(
        &self,
        x: (u128, u128),
        m: (u128, u128),
        a: (u128, u128),
        s: (u128, u128),
    ) -> (
        Option<(
            String,
            (u128, u128),
            (u128, u128),
            (u128, u128),
            (u128, u128),
        )>,
        (u128, u128),
        (u128, u128),
        (u128, u128),
        (u128, u128),
        u128,
    ) {
        let mut new_interval = ("".to_string(), x, m, a, s);
        let mut df_x = x;
        let mut df_m = m;
        let mut df_a = a;
        let mut df_s = s;
        let mut accepted_total = 0;

        match self.rating.as_str() {
            "x" => match self.operator {
                '<' => {
                    new_interval.1 = (x.0, self.quantity - 1);
                    df_x = (self.quantity, x.1);
                }
                '>' => {
                    new_interval.1 = (self.quantity + 1, x.1);
                    df_x = (x.0, self.quantity);
                }
                _ => panic!(),
            },
            "m" => match self.operator {
                '<' => {
                    new_interval.2 = (m.0, self.quantity - 1);
                    df_m = (self.quantity, m.1);
                }
                '>' => {
                    new_interval.2 = (self.quantity + 1, m.1);
                    df_m = (m.0, self.quantity);
                }
                _ => panic!(),
            },
            "a" => match self.operator {
                '<' => {
                    new_interval.3 = (a.0, self.quantity - 1);
                    df_a = (self.quantity, a.1);
                }
                '>' => {
                    new_interval.3 = (self.quantity + 1, a.1);
                    df_a = (a.0, self.quantity);
                }
                _ => panic!(),
            },
            "s" => match self.operator {
                '<' => {
                    new_interval.4 = (s.0, self.quantity - 1);
                    df_s = (self.quantity, s.1);
                }
                '>' => {
                    new_interval.4 = (self.quantity + 1, s.1);
                    df_s = (s.0, self.quantity);
                }
                _ => panic!(),
            },

            _ => panic!(),
        }

        let res = match &self.destination {
            Destination::Workflow(other) => {
                new_interval.0 = other.to_string();
                Some(new_interval)
            }
            Destination::Accepted => {
                let total_x = new_interval.1 .1 - new_interval.1 .0 + 1;
                let total_m = new_interval.2 .1 - new_interval.2 .0 + 1;
                let total_a = new_interval.3 .1 - new_interval.3 .0 + 1;
                let total_s = new_interval.4 .1 - new_interval.4 .0 + 1;

                accepted_total += total_s * total_m * total_x * total_a;
                None
            }
            Destination::Regected => None,
        };

        (res, df_x, df_m, df_a, df_s, accepted_total)
    }
}

#[derive(Debug, Clone)]
struct Piece {
    x: u128,
    m: u128,
    a: u128,
    s: u128,
}

#[derive(Debug, Clone)]
enum Destination {
    Accepted,
    Regected,
    Workflow(String),
}

fn calculate_accepted_pieces(input: &str) -> usize {
    let (_, (pieces, workflows)) = parse_input(input).expect("input malformed");

    let mut accepted_pieces: Vec<Piece> = vec![];

    for piece in pieces {
        let mut wf = "in".to_string();

        loop {
            match workflows.get(&wf).unwrap().get_destination(&piece) {
                Destination::Accepted => {
                    accepted_pieces.push(piece);
                    break;
                }
                Destination::Regected => break,
                Destination::Workflow(n_wf) => wf = n_wf,
            }
        }
    }

    accepted_pieces
        .iter()
        .map(|piece| piece.x + piece.m + piece.a + piece.s)
        .sum::<u128>() as usize
}

fn calculate_all_accepted_pieces(input: &str) -> usize {
    let (_, (_, workflows)) = parse_input(input).expect("input malformed");

    let mut q = VecDeque::new();
    q.push_back(("in".to_string(), (1, 4000), (1, 4000), (1, 4000), (1, 4000)));

    let mut all_accepted_pieces = 0;

    while !q.is_empty() {
        let (wf_name, x, m, a, s) = q.pop_back().unwrap();
        let wf = workflows.get(&wf_name).unwrap();

        let (mut new_intervals, accepted) = wf.calculate_new_intervals(x, m, a, s);

        q.append(&mut new_intervals);
        all_accepted_pieces += accepted as usize;
    }

    all_accepted_pieces
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[rstest]
    // #[case()]
    // fn test_func1(#[case] expected: u128, #[case] input: &str) {}

    #[test]
    fn test_func1() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}


{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";
        let expected = 19114;
        assert_eq!(expected, calculate_accepted_pieces(input));
    }

    #[test]
    fn test_func2() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}


{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";
        let expected = 167409079868000;
        assert_eq!(expected, calculate_all_accepted_pieces(input));
    }

    #[rstest]
    #[case(
        169600000000000,
        "in{s<1351:R,A}


{x=787,m=2655,a=1222,s=2876}
"
    )]
    #[case(
        78720000000000,
        "in{s<1351:R,qqz}
qqz{s>2770:A,m<1801:R,R}


{x=787,m=2655,a=1222,s=2876}
"
    )]

    fn test_small_test(#[case] expected: usize, #[case] input: &str) {
        assert_eq!(expected, calculate_all_accepted_pieces(input));
    }
}
