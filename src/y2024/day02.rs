use itertools::{enumerate, iterate};
use num::abs;
use rstest::rstest;
use std::time::Instant;

pub fn solve(input: &str) {
    let start_time = Instant::now();
    println!("First star: {}", star1(input));
    println!("\t time:{:?}", start_time.elapsed());

    let start_time = Instant::now();
    println!("Second star: {}", star2(input));
    println!("\t time:{:?}", start_time.elapsed());
}

fn star1(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let report : Vec<i32> = line
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            report
            }) //parse line to list of ints
        .filter(is_safe)
        .count()
        .try_into().unwrap()
}

fn is_safe(report: &Vec<i32>) -> bool {

    let diffs: Vec<i32> = report
        .windows(2)
        .map(|window| window[0] - window[1])
        .collect(); // find the differences between the levels in the reports

    diffs.iter()
        .fold(true, |r_is_safe, diff| r_is_safe && is_good_level(diffs[0], *diff)) // calculates if a report is safe given the two conditions programmed as invariants
}

fn is_good_level(first_diff: i32, diff:i32) -> bool {
    let positive_invariant = !((first_diff > 0) ^ (diff > 0));
    let interval_invariant = 1 <= abs(diff) && abs(diff) <= 3;
    positive_invariant && interval_invariant
}

fn star2(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let report : Vec<i32> = line
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            report
            }) //parse line to list of ints
        .filter(is_safe_problem_dampener)
        .count()
        .try_into()
        .unwrap()
}

fn is_safe_problem_dampener(report: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = report
        .windows(2)
        .map(|window| window[0] - window[1])
        .collect(); // find the differences between the levels in the reports
    

    let mut new_report1 = report.clone();
    let mut new_report2 = report.clone();
    let mut new_report3 = report.clone();
    let mut dampened = false;


    for (i,diff) in enumerate(diffs.clone()){
        if !is_good_level(diffs[0], diff){
            new_report1.remove((i + 1).try_into().unwrap());
            new_report2.remove((i).try_into().unwrap());
            new_report3.remove(0);
            dampened = true;
            break;
        } 
    }

    

    if dampened{
        let new_diffs: Vec<i32> = new_report1
            .windows(2)
            .map(|window| window[0] - window[1])
            .collect(); // find the differences between the levels in the reports


        let res1 = new_diffs.iter()
            .fold(true, |r_is_safe, diff| r_is_safe && is_good_level(new_diffs[0], *diff)); // calculates if a report is safe given the two conditions programmed as invariants

        let new_diffs: Vec<i32> = new_report2
            .windows(2)
            .map(|window| window[0] - window[1])
            .collect(); // find the differences between the levels in the reports


        let res2 = new_diffs.iter()
            .fold(true, |r_is_safe, diff| r_is_safe && is_good_level(new_diffs[0], *diff)); // calculates if a report is safe given the two conditions programmed as invariants

        let new_diffs: Vec<i32> = new_report3
            .windows(2)
            .map(|window| window[0] - window[1])
            .collect(); // find the differences between the levels in the reports


        let res3 = new_diffs.iter()
            .fold(true, |r_is_safe, diff| r_is_safe && is_good_level(new_diffs[0], *diff)); // calculates if a report is safe given the two conditions programmed as invariants


        res1 || res2 || res3
    } else{
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(2, "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9")]
    fn test_func1(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, star1(input));
    }

    #[rstest]
    #[case(4, "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9")]
    #[case(1, "85 85 82 80 79 78 76")]
    #[case(1, "79 81 77 74 72 71 70")]
    #[case(1, "49 53 51 52 53 55 56")]
    #[case(1, "62 61 62 63 65 67 68 71")]
    fn test_func2(#[case] expected: u32, #[case] input: &str) {
        assert_eq!(expected, star2(input));
    }

/*
report: [85, 85, 82, 80, 79, 78, 76], diffs: [0, 3, 2, 1, 1, 2], new_report: [85, 82, 80, 79, 78, 76], new_diffs [3, 2, 1, 1, 2], res false
report: [79, 81, 77, 74, 72, 71, 70], diffs: [-2, 4, 3, 2, 1, 1], new_report: [79, 77, 74, 72, 71, 70], new_diffs [2, 3, 2, 1, 1], res false
report: [49, 53, 51, 52, 53, 55, 56], diffs: [-4, 2, -1, -1, -2, -1], new_report: [53, 51, 52, 53, 55, 56], new_diffs [2, -1, -1, -2, -1], res false
report: [62, 61, 62, 63, 65, 67, 68, 71], diffs: [1, -1, -1, -2, -2, -1, -3], new_report: [62, 61, 63, 65, 67, 68, 71], new_diffs [1, -2, -2, -2, -1, -3], res false
*/
}
