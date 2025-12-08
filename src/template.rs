use std::time::Instant;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

#[tracing::instrument(skip(input))]
fn func1(input: &str) -> u32 {
    todo!()
}

#[tracing::instrument(skip(input))]
fn func2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use test_log::test;

    /*
    #[rstest]
    #[case()]
    fn test_func1(#[case] expected: u32, #[case] input: &str) {}
    */

    #[test]
    fn test_func1() {
        let input = "";
        let expected = 0;

        assert_eq!(func1(input), expected);
    }

    #[test]
    fn test_func2() {
        let input = "";
        let expected = 0;

        assert_eq!(func2(input), expected);
    }
}
