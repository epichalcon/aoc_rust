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

    #[test]
    fn test_func1() {}

    #[test]
    fn test_func2() {}
}
