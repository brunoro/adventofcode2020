use std::fs;
extern crate regex;

use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let line = "2-9 c: ccccccccc";
        let want = mk_policy_password(2, 9, 'c', "ccccccccc");
        let got = parse_line(line);
        assert_eq!(want.rule, got.rule);
        assert_eq!(want.letter, got.letter);
        assert_eq!(want.password, got.password);
    }

    #[test]
    fn example_by_len() {
        let inputs = vec![
            mk_policy_password(1, 3, 'a', "abcde"),
            mk_policy_password(1, 3, 'b', "cdefg"),
            mk_policy_password(2, 9, 'c', "ccccccccc"),
        ];
        let num_valid = super::count_valid_by_len(&inputs);
        assert_eq!(num_valid, 2);
    }

    #[test]
    fn example_by_pos() {
        let inputs = vec![
            mk_policy_password(1, 3, 'a', "abcde"),
            mk_policy_password(1, 3, 'b', "cdefg"),
            mk_policy_password(2, 9, 'c', "ccccccccc"),
        ];
        let num_valid = super::count_valid_by_pos(&inputs);
        assert_eq!(num_valid, 1);
    }
}

pub struct PolicyPassword<'a> {
    letter: char,
    rule: (usize, usize),
    password: &'a str,
}

pub fn mk_policy_password(min: usize, max: usize, letter: char, password: &str) -> PolicyPassword {
    PolicyPassword {
        letter: letter,
        rule: (min, max),
        password: password,
    }
}

fn parse_line<'a>(line: &'a str) -> PolicyPassword<'a> {
    lazy_static! {
        static ref RE: Regex = Regex::new("([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
    }
    let cap = RE.captures(line).unwrap();
    assert_eq!(cap.len(), 5);

    let min: usize = cap.get(1).map_or("", |m| m.as_str()).parse().unwrap();
    let max: usize = cap.get(2).map_or("", |m| m.as_str()).parse().unwrap();
    let letter: char = cap.get(3).map_or("", |m| m.as_str()).chars().nth(0).unwrap();
    let password: &str = cap.get(4).map_or("", |m| m.as_str());
    return mk_policy_password(min, max, letter, password);
}

pub fn count_valid_by_len(pps: &Vec<PolicyPassword>) -> usize {
    let mut count: usize = 0;
    for pp in pps {
        let (min, max) = pp.rule;
        let mut n: usize = 0;
        for c in pp.password.chars() { 
            if c == pp.letter {
                n += 1;
            }
        }
        if n >= min && n <= max {
            count += 1;
        }
    }
    return count;
}

pub fn count_valid_by_pos(pps: &Vec<PolicyPassword>) -> usize {
    let mut count: usize = 0;
    for pp in pps {
        let (a, b) = pp.rule;
        let has_a = pp.password.chars().nth(a - 1) == Some(pp.letter);
        let has_b = pp.password.chars().nth(b - 1) == Some(pp.letter);
        if has_a ^ has_b {
            count += 1;
        }
    }
    return count;
}

pub fn run() {
    let filename = "inputs/02.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let pps: Vec<PolicyPassword> = contents.lines().map(parse_line).collect();
    let by_len = count_valid_by_len(&pps);
    println!("{}", by_len);
    let by_pos = count_valid_by_pos(&pps);
    println!("{}", by_pos);
}
