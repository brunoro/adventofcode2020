use std::collections::HashMap;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_union() {
        let lines = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b", "",
        ];
        let groups = union_letters_per_group(&lines);
        assert_eq!(groups.len(), 5);
        assert_eq!(groups, vec![3, 3, 3, 1, 1]);
    }

    #[test]
    fn example_intersection() {
        let lines = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b", "",
        ];
        let groups = intersection_letters_per_group(&lines);
        assert_eq!(groups.len(), 5);
        assert_eq!(groups, vec![3, 0, 1, 1, 1]);
    }
}

pub fn union_letters_per_group(lines: &Vec<&str>) -> Vec<usize> {
    let mut counts: Vec<usize> = vec![];

    let mut curr_group: HashMap<char, usize> = HashMap::new();
    for line in lines.iter() {
        if line.len() == 0 {
            counts.push(curr_group.keys().count());
            curr_group = HashMap::new();
        } else {
            for c in line.chars() {
                let count: usize = match curr_group.get(&c) {
                    None => 1,
                    Some(val) => val+1
                };
                curr_group.insert(c, count);
            }
        }
    }
    return counts;
}

pub fn intersection_letters_per_group(lines: &Vec<&str>) -> Vec<usize> {
    let mut counts: Vec<usize> = vec![];

    let mut curr_group: HashMap<char, usize> = HashMap::new();
    let mut curr_group_len: usize = 0;
    for line in lines.iter() {
        if line.len() == 0 {
            let cnt = curr_group.values()
                .fold(0, |sum, v| if *v == curr_group_len { sum+1 } else { sum });
            counts.push(cnt);

            curr_group = HashMap::new();
            curr_group_len = 0;
        } else {
            for c in line.chars() {
                let count: usize = match curr_group.get(&c) {
                    None => 1,
                    Some(val) => val+1
                };
                curr_group.insert(c, count);
            }
            curr_group_len += 1;
        }
    }
    return counts;
}

pub fn run() {
    let filename = "inputs/06.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines: Vec<&str> = contents.lines().collect();
    lines.push(""); // add that extra line to validate the last passport

    let union_letters = union_letters_per_group(&lines);
    let union_sum: usize = union_letters.iter().sum();
    println!("{}", union_sum);

    let intersection_letters = intersection_letters_per_group(&lines);
    let intersection_sum: usize = intersection_letters.iter().sum();
    println!("{}", intersection_sum);
}
