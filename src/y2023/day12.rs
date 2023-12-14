use nom::{
    bytes::complete::tag,
    character::complete::{digit1, one_of},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use rstest::rstest;
use std::{cmp::min, collections::HashMap, time::Instant};

fn springs(input: &str) -> IResult<&str, String> {
    many1(one_of("?#."))(input).map(|(res, parsed)| (res, parsed.iter().collect::<String>()))
}

fn nums(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(","), digit1)(input).map(|(remaining, parsed)| {
        (
            remaining,
            parsed.into_iter().map(|s| s.parse().unwrap()).collect(),
        )
    })
}

fn parse_input(input: &str) -> IResult<&str, (String, Vec<usize>)> {
    separated_pair(springs, tag(" "), nums)(input)
}

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", input.lines().map(func1).sum::<usize>());
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", input.lines().map(func2).sum::<usize>());
    println!("\t time:{:?}", start_time.elapsed());
}

fn resolve_puzzle(
    mapping: &str,
    numbers: Vec<usize>,
    memo: &mut HashMap<(String, Vec<usize>), usize>,
) -> usize {
    if mapping == "" {
        if numbers.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    if numbers.is_empty() {
        if mapping.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }

    if let Some(res) = memo.get(&(mapping.to_string(), numbers.clone())).copied() {
        return res;
    }

    let mut result = 0;

    if mapping.starts_with('.') || mapping.starts_with('?') {
        result += resolve_puzzle(&mapping[1..], numbers.clone(), memo);
    }

    let first_number = numbers[0];

    if mapping.starts_with('#') || mapping.starts_with('?') {
        if first_number <= mapping.len()
            && !mapping[..first_number].contains('.')
            && (first_number == mapping.len() || mapping.chars().nth(first_number).unwrap() != '#')
        {
            let next_index = min(first_number + 1, mapping.len());
            result += resolve_puzzle(&mapping[next_index..], numbers[1..].to_vec(), memo)
        }
    }

    memo.insert((mapping.to_string(), numbers), result);

    return result;
}

fn func1(input: &str) -> usize {
    let (_, (springs, numbers)) = parse_input(input).expect("Input malformed");
    resolve_puzzle(&springs, numbers, &mut HashMap::new())
}

fn func2(input: &str) -> usize {
    let (_, (springs, numbers)) = parse_input(input).expect("Input malformed");
    let new_springs = vec![springs.clone(); 5].join("?");

    let mut new_numbers = vec![];

    for _ in 0..5 {
        new_numbers.append(&mut numbers.clone());
    }

    resolve_puzzle(&new_springs, new_numbers, &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(1, "???.### 1,1,3")]
    #[case(4, ".??..??...?##. 1,1,3")]
    #[case(1, "?#?#?#?#?#?#?#? 1,3,1,6")]
    #[case(1, "????.#...#... 4,1,1")]
    #[case(4, "????.######..#####. 1,6,5")]
    #[case(10, "?###???????? 3,2,1")]
    fn test_func1(#[case] expected: usize, #[case] input: &str) {
        assert_eq!(expected, func1(input));
    }

    #[rstest]
    #[case(0, "", vec![1])]
    #[case(0, ".", vec![1])]
    #[case(0, "#.", vec![])]
    #[case(1, "#", vec![1])]
    #[case(0, "#", vec![1, 1])]
    #[case(1, "##.", vec![2])]
    #[case(1, "##?", vec![2])]
    #[case(1, ".##", vec![2])]
    #[case(1, "#??.##", vec![2,2])]
    #[case(0, "#?#.##", vec![2,2])]
    #[case(1, "??.##", vec![2,2])]
    #[case(0, "??.##", vec![3,2])]
    #[case(1, "?", vec![1])]
    #[case(2, "??.", vec![1])]
    #[case(1, "??#.", vec![2])]
    #[case(1, "??#", vec![2])]
    #[case(2, "??", vec![1])]
    #[case(2, "???", vec![2])]
    #[case(1, "???", vec![1,1])]
    #[case(4, "??.??", vec![1,1])]
    #[case(4, "????", vec![1])]
    #[case(3, "???", vec![1])]
    #[case(2, "??", vec![1])]
    #[case(10, "???????", vec![2,1])]
    #[case(10, "?###????????", vec![3,2,1])]
    #[case(1, "??????????????###????????", vec![3])]
    #[case(10, "?###????????...?????????", vec![3,2,1,9])]
    fn test_resolve_puzzle(
        #[case] expected: usize,
        #[case] mapping: &str,
        #[case] numbers: Vec<usize>,
    ) {
        assert_eq!(
            expected,
            resolve_puzzle(mapping, numbers, &mut HashMap::new())
        );
    }

    #[test]
    fn test_func2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let expected = 525152;

        let result = input.lines().map(func2).sum::<usize>();

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(1, "???.### 1,1,3")]
    #[case(16384, ".??..??...?##. 1,1,3")]
    #[case(1, "?#?#?#?#?#?#?#? 1,3,1,6")]
    #[case(16, "????.#...#... 4,1,1")]
    #[case(2500, "????.######..#####. 1,6,5")]
    #[case(506250, "?###???????? 3,2,1")]
    fn test_func2_cases(#[case] expected: usize, #[case] input: &str) {
        assert_eq!(expected, func2(input));
    }
}
