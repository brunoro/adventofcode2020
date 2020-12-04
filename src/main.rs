#[macro_use] extern crate lazy_static;

mod day01;
mod day02;
mod day03;

fn main() {
    let days = [
        day01::run,
        day02::run,
        day03::run,
        ];
    let day = 3;
    days[day - 1]();
}
