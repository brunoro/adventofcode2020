use std::fs;

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let nums = vec![1721, 979, 366, 299, 675, 1456];
        let (a, b) = super::numbers_with_sum(nums, 2020);
        assert_eq!(a, 1721);
        assert_eq!(b, 299);
    }
}

pub fn numbers_with_sum(numbers: Vec<i32>, sum: i32) -> (i32, i32) {
    for i in 0..numbers.len() {
        for j in (i+1)..numbers.len() {
            if numbers[i] + numbers[j] == sum {
                return (numbers[i], numbers[j]);
            }
        }
    }
    println!("couldn't find numbers with sum={}", sum);
    return (0, 0);
}

pub fn run() {
    let filename = "inputs/01.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let nums: Vec<i32> = contents.lines().filter_map(|s| s.parse().ok()).collect();
    let (a, b) = numbers_with_sum(nums, 2020);
    println!("{}", a*b);
}
