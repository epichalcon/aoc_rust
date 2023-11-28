use std::fs;

pub fn read(day: u32) -> String {
    let path = format!("./src/inputs/day{:02}", day);
    fs::read_to_string(path).expect("Unable to read file")
}
