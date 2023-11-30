use regex::bytes::Regex;
pub fn get_i32_from_line(input: &str) -> Vec<i32> {
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(input.as_bytes())
        .map(|item| std::str::from_utf8(item.as_bytes()).unwrap())
        .map(|item| item.parse::<i32>().unwrap())
        .collect()
}
