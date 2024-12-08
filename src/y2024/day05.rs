use itertools::Itertools;
use nom::{character::complete::{char, newline, u32}, multi::{fold_many1, many0, separated_list0}, sequence::tuple, IResult};
use rstest::rstest;
use std::{collections::{hash_set, HashMap, HashSet}, time::Instant};

use crate::utils::direction;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn func1(input: &str) -> u32 {
    let (_, (dict, updates)) = parse_input(input).unwrap();

    updates.iter()
        .filter(|book_update| is_right_order(&book_update.to_vec(), &dict))
        .map(|book_update| book_update.get(book_update.len()/2).unwrap())
        .sum()
}

fn is_right_order(book_update: &Vec<u32>, dict: &HashMap<u32, HashSet<u32>>) -> bool {
    book_update.iter()
        .enumerate()
        .fold(true, |acc, (i, elem)| {
            if !(
                // the rest of the list as a set
                &book_update[(i+1)..]).iter()
                .cloned()
                .collect::<HashSet<_>>()

                //difference between the rest of the list and the numbers that are supposed to go
                //after
                .difference(
                    dict.get(elem)
                        .unwrap_or(&HashSet::new())
                ).collect::<Vec<_>>()

                .is_empty() {
                false
            }
            else{
                acc
            }
        })
}


/////////////////////second star////////////////////////////////////////////////////////////////


fn func2(input: &str) -> u32 {
    let (_, (dict, updates)) = parse_input(input).unwrap();

    updates.iter()
        .filter(|book_update| !is_right_order(&book_update.to_vec(), &dict))
        .map(|book_update| {
            *order_update(book_update, &dict)
                .get(book_update.len()/2).unwrap()
        })
        .sum()
}

fn order_update(book: &Vec<u32>, dict: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    // ordenar el vector con el orden proporcionado por el diccionario

    todo!()
}


        pub fn get_order(&self, node: &u8, stack: &mut Vec<u8>) {
            let receiving_nodes = self.graph.as_ref().unwrap().get(node);

                if receiving_nodes != None {
                    for value in receiving_nodes.unwrap() {
                        self.get_order(value, stack);
                    }
                }
                if !stack.contains(node) {
                    stack.push(*node);
                }
        }
/////////parsing///////////////////////////////////////

fn parse_input(input: &str) -> IResult<&str, (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>)> {
    let (input, dict) = fold_many1(
        rule , 
        HashMap::new, 
        |mut acc: HashMap<u32, HashSet<u32>>, (l, r)| {
            acc.entry(l).and_modify(|s| { s.insert(r); }).or_insert(HashSet::from([r]));
            acc
        }
    )(input)?;

    let (input, _) = newline(input)?;

    let (input, updates) = fold_many1(
        book_update, 
        Vec::new, 
        |mut acc, book| {
            acc.push(book);
            acc
        }
    )(input)?;

    Ok((input, (dict, updates)))
}

fn rule(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, (first, _, second, _)) = tuple((
        u32,
        char('|'),
        u32,
        newline
    ))(input)?;

    Ok((input, (first, second)))
}


fn book_update(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, list) = separated_list0(
        char(','), 
        u32
    )(input)?;

    let (input, _) = newline(input)?;

    Ok((input, list))
}

//testing///////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(143, "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47")]
    fn test_func1(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, func1(input))
    }

    #[rstest]
    #[case(123, "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
")]
    fn test_func2(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, func2(input))
    }
}
