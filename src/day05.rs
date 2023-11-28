use std::{collections::HashSet, time::Instant};

use regex::bytes::Regex;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!(
        "First star: {}",
        input.lines().filter(|x| is_nice(x)).count()
    );
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!(
        "Second star: {}",
        input.lines().filter(|x| is_nice2(x)).count()
    );
    println!("\t time:{:?}", start_time.elapsed());
}

fn is_nice(input: &str) -> bool {
    if Regex::new(r"ab|cd|pq|xy")
        .unwrap()
        .is_match(input.as_bytes())
    {
        return false;
    }

    let vowels = "aeiou";

    let mut prev = '0';
    let mut vowel_counter = 0;
    let mut double = false;
    for c in input.chars() {
        if c == prev {
            double = true
        }
        if vowels.contains(c) {
            vowel_counter += 1;
        }
        prev = c
    }

    double && vowel_counter >= 3
}

fn is_nice2(input: &str) -> bool {
    let mut pairs = HashSet::new();
    let mut pair_is_repeted = false;

    let _ = input
        .chars()
        .zip(input.chars().skip(1))
        .any(|pair| !pairs.insert(pair));

    let repetes = input
        .chars()
        .zip(input.chars().skip(2))
        .any(|pair| pair.0 == pair.1);

    for pair in pairs {
        let mut first_index = None;
        let str_pair: String = vec![pair.0, pair.1].into_iter().collect();

        for mat in Regex::new(&str_pair).unwrap().find_iter(input.as_bytes()) {
            if first_index == None {
                first_index = Some(mat.start());
            } else if first_index < Some(mat.start() - 1) {
                pair_is_repeted = true
            }
        }
    }

    pair_is_repeted && repetes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice_true() {
        assert!(is_nice("aaa"));
        assert!(is_nice("ugknbfddgicrmopn"));
    }

    #[test]
    fn test_is_nice_no_vowels() {
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_is_nice_no_doble() {
        assert!(!is_nice("jchzalrnumimnmhp"));
    }

    #[test]
    fn test_is_nice_has_substring() {
        assert!(!is_nice("haegwjzuvuyypxyu"));
    }

    #[test]
    fn test_is_nice2_true() {
        assert!(is_nice2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice2("xxyxx"));
        assert!(is_nice2("xyxy"));
        assert!(is_nice2("aabcdefegaa"));
        assert!(is_nice2("uiiii"));
    }

    #[test]
    fn test_is_nice2_nopair() {
        assert!(!is_nice2("ieodomkazucvgmuy"));
        assert!(!is_nice2("aaa"));
    }

    #[test]
    fn test_is_nice2_norepete() {
        assert!(!is_nice2("uurcxstgmygtbstg"));
    }
}
