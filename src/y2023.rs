mod day01;
mod day02;
use crate::utils::io;

pub fn run() {
    day01::solve(&io::read(2023, 1));
    day02::solve(&io::read(2023, 2))
}
