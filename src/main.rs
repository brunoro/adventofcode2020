#[macro_use] extern crate lazy_static;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let days = [
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        ];
    let day = 6;
    days[day - 1]();
}
