use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::many1,
    sequence::{delimited, separated_pair, tuple},
    IResult, Parser,
};
use num::Integer;
use rstest::rstest;
use std::{collections::BTreeMap, time::Instant};

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

fn parse_input(input: &str) -> IResult<&str, (&str, Vec<Node>)> {
    separated_pair(alpha1, tuple((newline, newline)), nodes)(input)
}

fn paths(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        tag("("),
        separated_pair(alpha1, tag(", "), alpha1),
        tuple((tag(")"), newline)),
    )(input)
}

fn node(input: &str) -> IResult<&str, Node> {
    separated_pair(alpha1, tag(" = "), paths)
        .map(|(name, paths)| Node {
            name: name.to_string(),
            left: paths.0.to_string(),
            right: paths.1.to_string(),
        })
        .parse(input)
}

fn nodes(input: &str) -> IResult<&str, Vec<Node>> {
    many1(node)(input)
}

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", traverse_map(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn traverse_map(input: &str) -> usize {
    let (_, (instructions, nodes_vec)) = parse_input(input).expect("Input malformed");
    let nodes = nodes_vec.iter().fold(BTreeMap::new(), |mut acc, node| {
        acc.insert(node.name.clone(), node);
        acc
    });

    let mut actual_node = nodes.get("AAA").unwrap();

    let mut i = 0;

    while actual_node.name != "ZZZ" {
        let path_to_take = instructions.chars().nth(i % instructions.len()).unwrap();

        match path_to_take {
            'R' => actual_node = nodes.get(&actual_node.right).unwrap(),
            'L' => actual_node = nodes.get(&actual_node.left).unwrap(),
            _ => panic!("input malformed"),
        };
        i += 1;
    }

    i
}

fn get_starting_nodes(nodes: &Vec<Node>) -> Vec<String> {
    nodes
        .iter()
        .map(|node| node.name.clone())
        .filter(|node| node.ends_with('A'))
        .collect()
}

fn is_ending_node(node: &String) -> bool {
    node.ends_with('Z')
}

fn func2(input: &str) -> usize {
    let (_, (instructions, nodes_vec)) = parse_input(input).expect("Input malformed");
    let nodes = nodes_vec.iter().fold(BTreeMap::new(), |mut acc, node| {
        acc.insert(node.name.clone(), node);
        acc
    });

    let starting_nodes = get_starting_nodes(&nodes_vec);

    let mut end_frequences = vec![];

    for mut node in starting_nodes.clone() {
        let mut i = 0;
        while !node.ends_with('Z') {
            let path_to_take = instructions.chars().nth(i % instructions.len()).unwrap();

            match path_to_take {
                'R' => node = nodes.get(&node).unwrap().right.clone(),
                'L' => node = nodes.get(&node).unwrap().left.clone(),
                _ => panic!("input malformed"),
            };
            i += 1;
        }
        end_frequences.push(i);
    }
    end_frequences
        .iter()
        .fold(1, |acc, frequence| acc.lcm(frequence))
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[rstest]
    // #[case()]
    // fn test_func1(#[case] expected: u32, #[case] input: &str) {}

    #[rstest]
    #[case(
        2,
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"
    )]
    #[case(
        6,
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"
    )]
    fn test_func1(#[case] result: usize, #[case] input: &str) {
        assert_eq!(result, traverse_map(input));
    }

    #[test]
    fn test_func2() {
        let input = "LR

AAA = (AAB, XXX)
AAB = (XXX, AAZ)
AAZ = (AAB, XXX)
BBA = (BBB, XXX)
BBB = (BBC, BBC)
BBC = (BBZ, BBZ)
BBZ = (BBB, BBB)
XXX = (XXX, XXX)
CCA = (CCB, XXX)
CCB = (XXX, CCC)
CCC = (CCD, XXX)
CCD = (XXX, CCE)
CCE = (CCZ, XXX)
CCZ = (XXX, CCA)
";
        let result = 30;
        assert_eq!(result, func2(input));
    }
}
