use std::fs;

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let nums = vec![1721, 979, 366, 299, 675, 1456];
        let (a, b) = super::two_numbers_with_sum(&nums, 2020);
        assert_eq!(a, 1721);
        assert_eq!(b, 299);
    }
}

pub fn two_numbers_with_sum(nums: &Vec<i32>, sum: i32) -> (i32, i32) {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == sum {
                return (nums[i], nums[j]);
            }
        }
    }
    println!("couldn't find two numbers with sum={}", sum);
    return (0, 0);
}

pub fn three_numbers_with_sum(nums: &Vec<i32>, sum: i32) -> (i32, i32, i32) {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            for k in (j + 1)..nums.len() {
                if nums[i] + nums[j] + nums[k] == sum {
                    return (nums[i], nums[j], nums[k]);
                }
            }
        }
    }
    println!("couldn't find three numbers with sum={}", sum);
    return (0, 0, 0);
}

pub fn run() {
    let filename = "inputs/01.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let nums: Vec<i32> = contents.lines().filter_map(|s| s.parse().ok()).collect();
    let (a, b) = two_numbers_with_sum(&nums, 2020);
    println!("{}", a * b);
    let (c, d, e) = three_numbers_with_sum(&nums, 2020);
    println!("{}", c * d * e);
}
