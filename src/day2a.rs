use std::fs;

fn is_safe_report(nums: &[i32]) -> bool {
    if nums.len() < 2 {
        return true;
    }

    let initial_increasing = nums[0] <= nums[1];

    nums.windows(2)
        .all(|window| {
            let current_increasing = window[0] <= window[1];
            if current_increasing != initial_increasing {
                return false;
            }

            let diff = (window[1] - window[0]).abs();
            diff >= 1 && diff <= 3
        })
}

pub fn main() {
    let input = fs::read_to_string("input/2.txt").expect("Unable to read input file");

    let ans = input
        .lines()
        .filter(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            is_safe_report(&nums)
        })
        .count();

    println!("{:?}", ans);
}
