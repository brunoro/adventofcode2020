use std::fs;
use std::convert::TryInto;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (row, col) = find_seat("FBFBBFFRLR");
        assert_eq!(row, 44);
        assert_eq!(col, 5);
    }
}

type Seat = (u32, u32);

fn pow2(x: usize) -> u32 {
    let base: u32 = 2;
    return base.pow(x.try_into().unwrap());
}

pub fn find_seat(s: &str) -> Seat {
    assert_eq!(s.len(), 10);
    let mut row = 0;
    for (i, c) in s[0..7].chars().enumerate() {
        if c == 'B' {
            row += pow2(6 - i);
        }
    }
    let mut col = 0;
    for (i, c) in s[7..10].chars().enumerate() {
        if c == 'R' {
            col+= pow2(2 - i);
        }
    }
    return (row, col);
}

fn seat_id((row, col): Seat) -> u32 {
    return row * 8 + col;
}

fn missing_seat_id(seats: &Vec<u32>) -> u32 {
    let mut i = *(seats.first().unwrap());
    for s in seats {
        if i != *s {
            break;
        }
        i += 1;
    }
    return i;
}

pub fn run() {
    let filename = "inputs/05.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut seats: Vec<u32> = contents.lines().map(find_seat).map(seat_id).collect();
    seats.sort();
    // max
    println!("{}", seats.last().unwrap());
    // missing
    println!("{}", missing_seat_id(&seats));
}
