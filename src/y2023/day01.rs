use regex::Regex;
use std::time::Instant;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!(
        "First star: {}",
        input.lines().map(get_coordinates).sum::<i32>()
    );
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!(
        "Second star: {}",
        input.lines().map(get_spelled_coordinates).sum::<i32>()
    );
    println!("\t time:{:?}", start_time.elapsed());
}

fn get_coordinates(input: &str) -> i32 {
    let digits: Vec<i32> = Regex::new(r"\d")
        .unwrap()
        .find_iter(input)
        .map(|digit| digit.as_str().parse::<i32>().unwrap())
        .collect();

    if digits.len() == 1 {
        digits[0] * 10 + digits[0]
    } else {
        digits[0] * 10 + digits[digits.len() - 1]
    }
}

fn get_spelled_coordinates(input: &str) -> i32 {
    let pattern = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut i = 0;
    let mut digits = vec![];

    while i < input.len() {
        let mat_res = pattern.find(&input[i..]);
        match mat_res {
            Some(mat) => {
                i += mat.start();
                digits.push(transform_spelling_to_int(mat.as_str()));
            }
            None => (),
        }
        i += 1;
    }

    let digit1 = digits[0];
    let digit2 = digits[digits.len() - 1];

    digit1 * 10 + digit2
}

fn transform_spelling_to_int(spelling: &str) -> i32 {
    match spelling {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => spelling.parse::<i32>().expect("Expected int, got string"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_coords_one_number() {
        assert_eq!(77, get_coordinates("treb7uchet"));
    }

    #[test]
    fn test_get_coords_two_numbers() {
        assert_eq!(12, get_coordinates("1asdf2"));
        assert_eq!(24, get_coordinates("24"));
    }

    #[test]
    fn test_get_coords_multiple_numbers() {
        assert_eq!(15, get_coordinates("a1b2c3d4e5f"));
    }

    #[test]
    fn test_get_coords_several_inputs() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, input.lines().map(get_coordinates).sum());
    }

    #[test]
    fn test_get_spelled_coords_multiple_numbers() {
        assert_eq!(15, get_spelled_coordinates("abcone2fivexyz"));
        assert_eq!(29, get_spelled_coordinates("two1nine"));
        assert_eq!(83, get_spelled_coordinates("eightwothree"));
        assert_eq!(64, get_spelled_coordinates("xsixne3four"));
        assert_eq!(77, get_spelled_coordinates("seven"));
        assert_eq!(42, get_spelled_coordinates("4nineeightsevenw2"));
        assert_eq!(42, get_spelled_coordinates("ni4n2e"));
        assert_eq!(29, get_spelled_coordinates("twosevenine"));
    }

    #[test]
    fn test_get_spelled_coords_several_inputs() {
        let input = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightsevenw2
    zoneight234
    7pqrstsixteen";
        assert_eq!(281, input.lines().map(get_spelled_coordinates).sum());
    }
}
