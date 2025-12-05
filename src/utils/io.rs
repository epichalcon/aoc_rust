use std::{fs, io::{self, Write}, thread::sleep, time::Duration};

pub fn read(year: u32, day: u32) -> String {
    let path = format!("./src/inputs/y{}/day{:02}", year, day);
    println!("{}", path);
    fs::read_to_string(path).expect("Unable to read file")
}

pub fn animate(input: &str) {
    print!("\r{}", input);
    io::stdout().flush().unwrap();
    sleep(Duration::from_millis(100));
}
