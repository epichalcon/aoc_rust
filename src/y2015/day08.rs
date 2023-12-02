use rstest::rstest;
use std::time::Instant;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!(
        "First star: {}",
        input.lines().map(difference_of_characters).sum::<usize>()
    );
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2());
    println!("\t time:{:?}", start_time.elapsed());
}

fn difference_of_characters(input: &str) -> usize {
    let initial_len = input.len() + 2;
    let modified_input = input.replace(r#"\""#, "'").replace(r#"\\"#, "-");

    let hex_cars_num = input.find(r#"\x"#).unwrap_or(0);

    initial_len - modified_input.len() - 3 * hex_cars_num
}
fn func2() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(2, r#""""#)]
    #[case(2, r#""abc""#)]
    #[case(3, r#""aaa\"aaa""#)]
    #[case(5, r#""\x27""#)]
    fn test_get_spelled_coords_multiple_numbers(#[case] expected: usize, #[case] input: &str) {
        assert_eq!(expected, difference_of_characters(input));
    }

    #[test]
    fn test_func2() {}
}
