use core::str;
// this day will be programmed using minizinc the code is in mz/day13.mzn
//
use std::{ fs::File, io::Write, process::Command, time::Instant};

use nom::{branch::alt, bytes::{complete::tag}, character::complete::{char, newline, u32}, multi::{fold_many1, many0, many1, separated_list0}, sequence::tuple, AsBytes, IResult};


pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", func1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", func2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn func1(input: &str) -> u32 {
    let (_, configurations) = parse_input(input).unwrap();

    configurations.iter()
        .map(|(a, b, p)| calculate_tokens(a, b, p).unwrap())
        .sum()
}

fn calculate_tokens(a: &(u32, u32), b: &(u32, u32), p: &(u32, u32)) -> Result<u32, std::io::Error> {
    let mut data = File::create("src/y2024/mz/data.dzn")?;

    let string_data = format!("Ax = {0};
Ay = {1};
Bx = {2};
By = {3};
Prizex = {4};
Prizey = {5};", a.0, a.1, b.0, b.1, p.0, p.1);

    let _ = data.write_all(string_data.as_bytes());


    match mz_call() {
        Some((a, b)) => Ok(a*3 + b),
        None => Ok(0)
    }

}

fn mz_call() -> Option<(u32, u32)> {
    //let command = "MiniZincIDE-2.6.4-bundle-linux-x86_64/bin/minizinc --solver Chuffed day13.mzn data.dzn";

    let minizinc_path = "src/y2024/mz/MiniZincIDE-2.6.4-bundle-linux-x86_64/bin/minizinc";

    // The arguments to pass to MiniZinc
    let args = [
        "--solver", "Chuffed", 
        "src/y2024/mz/day13.mzn", 
        "src/y2024/mz/data.dzn"
    ];

    // Run the MiniZinc command
    let output = Command::new(minizinc_path)
        .args(&args)
        .output() // Executes the command and returns the status
        .expect("Failed to execute MiniZinc");

    // Check if the command was successful
    if !output.status.success() {
        eprintln!("MiniZinc command failed with status: {}", output.status);
    }

    // Convert stdout to a string and print it
    let stdout = str::from_utf8(&output.stdout).unwrap_or("Invalid UTF-8 in stdout");
    println!("MiniZinc Output: \n{}", stdout);

    let (_, res) = parse_mz_output(stdout).unwrap();
    res
}


fn func2(input: &str) -> bool {
    todo!()
}

//parsing/////////////////////////////////////////////////////////////////////////////////


fn parse_input(input: &str) -> IResult<&str, Vec<((u32, u32), (u32, u32), (u32, u32))>> {
    many1(configuration)(input)
}

fn configuration(input: &str) -> IResult<&str, ((u32, u32), (u32, u32), (u32, u32))> {
    let (input, (_, ax, _, ay, _)) = tuple((
        tag("Button A: X+"),
        u32,
        tag(", Y+"),
        u32,
        newline
    ))(input)?;

    let (input, (_, bx, _, by, _)) = tuple((
        tag("Button B: X+"),
        u32,
        tag(", Y+"),
        u32,
        newline
    ))(input)?;


    let (input, (_, px, _, py, _)) = tuple((
        tag("Prize: X="),
        u32,
        tag(", Y="),
        u32,
        newline
    ))(input)?;

    let (input, _) = newline(input)?;

    Ok((input, ((ax, ay), (bx, by), (px, py))))
}


fn parse_mz_output(input: &str) ->IResult<&str, Option<(u32, u32)>> { 
    alt((correct_output, unsat))(input)
}

fn correct_output(input: &str) -> IResult<&str, Option<(u32, u32)>> {
    let (input, (_, a, _, b)) = tuple((
        tag("a = "),
        u32,
        tag(";\nb = "),
        u32
    ))(input)?;

    Ok((input, Some((a,b))))
}

fn unsat(input: &str) -> IResult<&str, Option<(u32, u32)>>{
    let (input, _) = tag("=====UNSATISFIABLE=====")(input)?;

    Ok((input, None))
}


//testing/////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(480, "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279

")]
    fn test_func1(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, func1(input))
    }

    #[test]
    fn test_func2() {}
}
