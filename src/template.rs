use std::time::Instant;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn func1(input: &str) -> bool {
    todo!()
}
fn func2(input: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case()]
    fn test_func1(#[case] expected: u32, #[case] input: &str) {}

    #[test]
    fn test_func2() {}
}
