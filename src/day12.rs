use regex::Regex;
use std::time::Instant;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", sum_numbers(&input));
    println!("\t{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", sum_numbers_not_red(&input));
    println!("\t{:?}", start_time.elapsed());
}

fn sum_numbers(input: &str) -> i32 {
    let mut result = 0;
    for number_match in Regex::new(r"\d+|-\d+").unwrap().find_iter(input) {
        if let Ok(number) = number_match.as_str().parse::<i32>() {
            result += number;
        }
    }
    result
}

fn sum_numbers_not_red(input: &str) -> i32 {
    let no_red = Regex::new(r"\{.*(\[.*\])*[^\[]*?red[^\]]*(\[.*\])*.*\}")
        .unwrap()
        .replace_all(input, "");

    println!("{}", no_red);
    sum_numbers(&no_red)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_number_basic() {
        assert_eq!(6, sum_numbers("[1,2,3]"));
        assert_eq!(6, sum_numbers(r#"{"a":2,"b":4}"#));
        assert_eq!(3, sum_numbers("[[[3]]]"));
    }

    #[test]
    fn test_sum_number_negative() {
        assert_eq!(3, sum_numbers(r#"{"a":{"b":4},"c":-1}"#));
        assert_eq!(0, sum_numbers(r#"{"a":[-1,1]}"#));
        assert_eq!(0, sum_numbers(r#"[-1,{"a":1}]"#));
    }
    #[test]
    fn test_sum_number_empty() {
        assert_eq!(0, sum_numbers("[]"));
        assert_eq!(0, sum_numbers("{}"));
    }

    #[test]
    fn test_not_red_normal() {
        assert_eq!(6, sum_numbers_not_red("[1,2,3]"));
    }

    #[test]
    fn test_not_red_eliminate_object() {
        assert_eq!(4, sum_numbers_not_red(r#"[1,{"c":"red","b":2},3]"#));
    }

    #[test]
    fn test_not_red_eliminate_all() {
        assert_eq!(0, sum_numbers_not_red(r#"{"d":"red","e":[1,2,3,4],"f":5}"#));
    }

    #[test]
    fn test_not_red_no_object() {
        assert_eq!(6, sum_numbers_not_red(r#"[1,"red",5]"#));
    }

    #[test]
    fn test_not_red_array_in_object() {
        assert_eq!(5, sum_numbers_not_red(r#"{"e":[1,"red",5], "f": 5}"#));
    }
}
