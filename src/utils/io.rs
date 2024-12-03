use std::fs;

pub fn read(year: u32, day: u32) -> String {
    let path = format!("./src/inputs/y{}/day{:02}", year, day);
    println!("{}", path);
    fs::read_to_string(path).expect("Unable to read file")
}
