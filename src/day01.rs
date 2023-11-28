use std::time::Instant;

pub fn solve(input: &str) {
    let start_time: Instant = Instant::now();
    println!("First star: {}", compute_parenthesies(&input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time: Instant = Instant::now();
    println!("Second star: {}", enter_the_basement(&input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn compute_parenthesies(input: &str) -> i32 {
    let mut open = 0;
    let mut close = 0;
    for c in input.chars() {
        match c {
            '(' => open += 1,
            ')' => close += 1,
            _ => (),
        }
    }
    open - close
}

fn enter_the_basement(input: &str) -> i32 {
    let mut i = 0;
    let mut level = 0;
    for c in input.chars() {
        i += 1;
        match c {
            '(' => level += 1,
            ')' => level -= 1,
            _ => (),
        }

        if level < 0 {
            return i;
        }
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(0, compute_parenthesies("(())"));
        assert_eq!(0, compute_parenthesies("()()"));

        assert_eq!(3, compute_parenthesies("((("));
        assert_eq!(3, compute_parenthesies("(()(()("));
        assert_eq!(3, compute_parenthesies("))((((("));

        assert_eq!(-1, compute_parenthesies("())"));
        assert_eq!(-1, compute_parenthesies("))("));

        assert_eq!(-3, compute_parenthesies(")))"));
        assert_eq!(-3, compute_parenthesies(")())())"));
    }

    #[test]
    fn test_star_two() {
        assert_eq!(1, enter_the_basement(")"));
        assert_eq!(5, enter_the_basement("()())"));
    }
}
