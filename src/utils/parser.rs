use core::fmt::Debug;
use core::str::FromStr;
use regex::bytes::Regex;

pub fn get_i32_from_line(input: &str) -> Vec<i32> {
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(input.as_bytes())
        .map(|item| std::str::from_utf8(item.as_bytes()).unwrap())
        .map(|item| item.parse::<i32>().unwrap())
        .collect()
}

pub fn get_number_from_line<T: std::str::FromStr>(input: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(input.as_bytes())
        .map(|item| std::str::from_utf8(item.as_bytes()).unwrap())
        .map(|item| item.parse::<T>().unwrap())
        .collect()
}

pub fn get_u128_from_line(input: &str) -> Vec<u128> {
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(input.as_bytes())
        .map(|item| std::str::from_utf8(item.as_bytes()).unwrap())
        .map(|item| item.parse::<u128>().unwrap())
        .collect()
}
