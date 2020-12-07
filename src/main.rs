#[macro_use] extern crate lazy_static;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let days = [
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        ];
    let day = 4;
    days[day - 1]();
}
