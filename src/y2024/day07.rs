use std::time::Instant;

use nom::{character::complete::{char, newline, u64}, multi::many1, sequence::tuple, IResult};

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}


//first star////////////////////////////////////////////////////////

fn func1(input: &str) -> u64 {
    let (_, equations) = parse_input(input).unwrap();


    equations.iter()
        .filter_map(|equation| 
            if is_correct(equation){
                Some(equation.0)
            } else {
                None
            }
        )
        .sum()
}

fn is_correct(equation: &(u64, Vec<u64>)) -> bool {
    let res = equation.0;
    let nums = equation.1.clone();
    let operators = generate_combinations(nums.len());

    operators.into_iter()
        .filter(|operator| {
            res == solve_operation(&nums, operator)
        }).count() > 0

}

fn generate_combinations(n: usize) -> Vec<Vec<bool>> {
        let mut result = Vec::new();
    
    // There are 2^n combinations (from 0 to 2^n - 1).
    for i in 0..(1 << n) {
        let mut combination = Vec::new();
        
        for j in 0..n {
            // Check if the j-th bit of i is 1 (true) or 0 (false)
            combination.push((i >> j) & 1 == 1);
        }
        
        result.push(combination);
    }
    
    result
}

fn solve_operation(nums: &Vec<u64>, operators: &Vec<bool>) -> u64 {
    let res = nums.iter()
        .skip(1)
        .zip(operators.iter())
        .fold(*nums.get(0).unwrap(), |acc, (num, op)| {
            if *op {
                acc + num
            } else {
                acc * num
            }
        });
    res
}

//second star////////////////////////////////////////////////////////


fn func2(input: &str) -> bool {
    todo!()
}


//parser////////////////////////////////////////////////////////

fn parse_input(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    many1(equation)(input)
}


fn equation(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    let (input, (res, _colon)) = tuple((
        u64,
        char(':')
    ))(input)?;

    let (input, numbers) = many1(
        tuple((
            char(' '),
            u64
        ))
    )(input)?;

    let (input, _) = newline(input)?;

    let numbers = numbers.into_iter().map(|(_, n)| n).collect();
    Ok((input, (res, numbers)))
}


//tests////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[test]
    fn test_func1() {
        let expected = 3749;
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
        assert_eq!(expected, func1(input))
    }

    #[test]
    fn test_func2() {}
}
