use rstest::rstest;
use std::time::Instant;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1());
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2());
    println!("\t time:{:?}", start_time.elapsed());
}

fn func1() -> bool {
    true
}
fn func2() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case()]
    fn test_get_spelled_coords_multiple_numbers(#[case] expected: i32, #[case] input: &str) {}

    #[test]
    fn test_func2() {}
}
