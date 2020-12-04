#[macro_use] extern crate lazy_static;

mod day01;
mod day02;

fn main() {
    let days = [day01::run, day02::run];
    let day = 2;
    days[day - 1]();
}
