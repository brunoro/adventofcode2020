use std::collections::HashMap;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_complete() {
        let lines = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
            "",
        ];
        let num_valid = count_complete_passports(&lines);
        assert_eq!(num_valid, 2);
    }

    #[test]
    fn example_invalid() {
        let lines = vec![
            "eyr:1972 cid:100",
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "",
            "iyr:2019",
            "hcl:#602927 eyr:1967 hgt:170cm",
            "ecl:grn pid:012533040 byr:1946",
            "",
            "hcl:dab227 iyr:2012",
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "",
            "hgt:59cm ecl:zzz",
            "eyr:2038 hcl:74454a iyr:2023",
            "pid:3556412378 byr:2007",
            "",
        ];
        let num_valid = count_valid_passports(&lines);
        assert_eq!(num_valid, 0);
    }

    #[test]
    fn example_valid() {
        let lines = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
            "hcl:#623a2f",
            "",
            "eyr:2029 ecl:blu cid:129 byr:1989",
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "",
            "hcl:#888785",
            "hgt:164cm byr:2001 iyr:2015 cid:88",
            "pid:545766238 ecl:hzl",
            "eyr:2022",
            "",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
            "",
        ];
        let num_valid = count_complete_passports(&lines);
        assert_eq!(num_valid, 4);
    }
}

type Passport = HashMap<String, Option<String>>;

fn new_passport() -> Passport {
    let mut map: HashMap<String, Option<String>> = HashMap::new();
    map.insert("byr".to_string(), None);
    map.insert("iyr".to_string(), None);
    map.insert("eyr".to_string(), None);
    map.insert("hgt".to_string(), None);
    map.insert("hcl".to_string(), None);
    map.insert("ecl".to_string(), None);
    map.insert("pid".to_string(), None);
    return map;
}

fn is_complete_passport(p: Passport) -> bool {
    for (_, field) in p.iter() {
        if field.is_none() {
            return false;
        }
    }
    return true;
}

fn is_valid_year(s: &str, min: u32, max: u32) -> bool {
    if s.len() != 4 {
        return false;
    }
    let n: u32 = s.parse().unwrap();
    return n >= min && n <= max;
}

fn is_valid_height(s: &str) -> bool {
    let len = s.len();
    if len <= 2 {
        return false;
    }
    let n: u32 = s[0..(len - 2)].parse().unwrap();
    if s.ends_with("cm") {
        return n >= 150 && n <= 193;
    }
    if s.ends_with("in") {
        return n >= 59 && n <= 76;
    }
    return false;
}

fn is_valid_hex(s: &str) -> bool {
    if s.chars().nth(0).unwrap() != '#' {
        return false;
    }
    for c in s[1..s.len()].chars() {
        if !((c >= 'a' && c <= 'f') || (c >= '0' && c <= '9')) {
            return false;
        }
    }
    return true;
}

fn is_valid_eye_color(s: &str) -> bool {
    return s == "amb"
        || s == "blu"
        || s == "brn"
        || s == "gry"
        || s == "grn"
        || s == "hzl"
        || s == "oth";
}

fn is_valid_digit_seq(s: &str) -> bool {
    if s.len() != 9 {
        return false;
    }
    for c in s.chars() {
        if c < '0' || c > '9' {
            return false;
        }
    }
    return true;
}

fn is_valid_field(k: &str, v: &str) -> bool {
    return match k {
        // (Birth Year) - four digits; at least 1920 and at most 2002.
        "byr" => is_valid_year(v, 1920, 2002),
        // (Issue Year) - four digits; at least 2010 and at most 2020.
        "iyr" => is_valid_year(v, 2010, 2020),
        // (Expiration Year) - four digits; at least 2020 and at most 2030.
        "eyr" => is_valid_year(v, 2020, 2030),
        // (Height) - a number followed by either cm or in:
        // If cm, the number must be at least 150 and at most 193.
        // If in, the number must be at least 59 and at most 76.
        "hgt" => is_valid_height(v),
        // (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        "hcl" => is_valid_hex(v),
        // (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        "ecl" => is_valid_eye_color(v),
        // (Passport ID) - a nine-digit number, including leading zeroes.
        "pid" => is_valid_digit_seq(v),
        //  default case
        _ => false,
    };
}

fn is_valid_passport(p: Passport) -> bool {
    for (k, entry) in p.iter() {
        let valid = match entry {
            None => false,
            Some(v) => is_valid_field(k, v),
        };
        if !valid {
            return false;
        }
    }
    return true;
}

pub fn count_complete_passports(lines: &Vec<&str>) -> usize {
    let mut count: usize = 0;
    let mut current_passport = new_passport();
    for line in lines.iter() {
        if line.len() == 0 {
            if is_complete_passport(current_passport) {
                count += 1;
            }
            current_passport = new_passport();
        } else {
            let pairs: Vec<&str> = line.split_ascii_whitespace().collect();
            for p in pairs {
                let kv: Vec<&str> = p.split(':').collect();
                assert_eq!(kv.len(), 2);
                let key = kv[0].to_string();
                let value = kv[1].to_string();
                if current_passport.contains_key(&key) {
                    current_passport.insert(key, Some(value));
                }
            }
        }
    }
    return count;
}

pub fn count_valid_passports(lines: &Vec<&str>) -> usize {
    let mut count: usize = 0;
    let mut current_passport = new_passport();
    for line in lines.iter() {
        if line.len() == 0 {
            if is_valid_passport(current_passport) {
                count += 1;
            }
            current_passport = new_passport();
        } else {
            let pairs: Vec<&str> = line.split_ascii_whitespace().collect();
            for p in pairs {
                let kv: Vec<&str> = p.split(':').collect();
                assert_eq!(kv.len(), 2);
                let key = kv[0].to_string();
                let value = kv[1].to_string();
                if current_passport.contains_key(&key) {
                    current_passport.insert(key, Some(value));
                }
            }
        }
    }
    return count;
}

pub fn run() {
    let filename = "inputs/04.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines: Vec<&str> = contents.lines().collect();
    lines.push(""); // add that extra line to validate the last passport
    println!("{}", count_complete_passports(&lines));
    println!("{}", count_valid_passports(&lines));
}
