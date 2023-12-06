use core::fmt::Debug;
use core::str::FromStr;
use regex::bytes::Regex;

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
