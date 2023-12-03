mod day01;
mod day02;
mod day03;
use crate::utils::io;

pub fn run() {
    day01::solve(&io::read(2023, 1));
    day02::solve(&io::read(2023, 2));
    day03::solve(&io::read(2023, 3));
}
