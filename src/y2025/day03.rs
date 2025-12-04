use std::time::Instant;

use nom::branch;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn to_number(n1: u32, n2: u32) -> u32 {
    n1 * 10 + n2
}

fn func1(input: &str) -> u32 {
    input
        .lines()
        .map(|bank| {
            let (n1, n2) = bank.chars().fold((0, 0), |(n1, n2), battery| {
                let battery_num = battery.to_digit(10).unwrap();

                if to_number(n2, battery_num) > to_number(n1, n2) {
                    (n2, battery_num)
                } else if battery_num > n2 {
                    (n1, battery_num)
                } else {
                    (n1, n2)
                }
            });
            to_number(n1, n2)
        })
        .sum()
}

fn to_number_vec(numbers: &[u64]) -> u64 {
    numbers.iter().fold(0, |acc, n| acc * 10 + n)
}

fn func2(input: &str) -> u64 {
    input
        .lines()
        .map(|bank| {
            let activated: Vec<u64> = bank.chars().fold(vec![0; 12], |activated, battery| {
                let battery_num = battery.to_digit(10).unwrap().into();

                let mut new_activated = activated.clone();

                for i in 0..12 {
                    let mut shift_vec = activated.clone();
                    shift_vec.remove(i);
                    shift_vec.push(battery_num);

                    if to_number_vec(&shift_vec) > to_number_vec(&new_activated) {
                        new_activated = shift_vec;
                    } 
                }
                new_activated 
            });

            dbg!(to_number_vec(&activated));
            to_number_vec(&activated)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func1() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let expected = 357;

        assert_eq!(func1(input), expected);
    }

    #[test]
    fn test_func2() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let expected = 3121910778619;

        assert_eq!(func2(input), expected);
    }
}
