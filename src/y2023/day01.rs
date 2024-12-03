use rstest::rstest;
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
    let mut digits = input.chars().filter_map(|n| n.to_digit(10));

    let first = digits.next().expect("Should be a number");

    match digits.last() {
        Some(last) => format!("{first}{last}"),
        None => format!("{first}{first}"),
    }
    .parse::<i32>()
    .expect("Should be a number")
}

fn get_spelled_coordinates(input: &str) -> i32 {
    let mut digits = (0..input.len()).filter_map(|index| {
        let reduced_input = &input[index..];
        let result = if reduced_input.starts_with("one") {
            '1'
        } else if reduced_input.starts_with("two") {
            '2'
        } else if reduced_input.starts_with("three") {
            '3'
        } else if reduced_input.starts_with("four") {
            '4'
        } else if reduced_input.starts_with("five") {
            '5'
        } else if reduced_input.starts_with("six") {
            '6'
        } else if reduced_input.starts_with("seven") {
            '7'
        } else if reduced_input.starts_with("eight") {
            '8'
        } else if reduced_input.starts_with("nine") {
            '9'
        } else {
            reduced_input.chars().next().unwrap()
        };

        result.to_digit(10)
    });

    let first = digits.next().expect("Should be a number");

    match digits.last() {
        Some(last) => format!("{first}{last}"),
        None => format!("{first}{first}"),
    }
    .parse::<i32>()
    .expect("Should be a number")
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

    #[rstest]
    #[case(12, "1asdf2")]
    #[case(24, "24")]
    fn test_get_coords_two_numbers(#[case] expected: i32, #[case] input: &str) {
        assert_eq!(expected, get_coordinates(input));
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

    #[rstest]
    #[case(15, "abcone2fivexyz")]
    #[case(29, "two1nine")]
    #[case(83, "eightwothree")]
    #[case(64, "xsixne3four")]
    #[case(77, "seven")]
    #[case(42, "4nineeightsevenw2")]
    #[case(42, "ni4n2e")]
    #[case(29, "twosevenine")]
    fn test_get_spelled_coords_multiple_numbers(#[case] expected: i32, #[case] input: &str) {
        assert_eq!(expected, get_spelled_coordinates(input));
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
