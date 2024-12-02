use std::fs;

fn is_safe_report_with_skip(nums: &[i32], skip_index: Option<usize>) -> bool {
    if nums.len() <= 2 {
        return true;
    }

    let first_pair: Vec<i32> = nums.iter()
        .enumerate()
        .filter(|&(i, _)| Some(i) != skip_index)
        .take(2)
        .map(|(_, &num)| num)
        .collect();

    if first_pair.len() < 2 {
        return true;
    }

    let initial_increasing = first_pair[0] <= first_pair[1];

    nums.iter()
        .enumerate()
        .filter(|&(i, _)| Some(i) != skip_index)
        .zip(nums.iter().enumerate().filter(|&(i, _)| Some(i) != skip_index).skip(1))
        .all(|((_, &num1), (_, &num2))| {
            let current_increasing = num1 <= num2;
            if current_increasing != initial_increasing {
                return false;
            }

            let diff = (num2 - num1).abs();
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
            
            is_safe_report_with_skip(&nums, None) ||
            (0..nums.len()).any(|i| is_safe_report_with_skip(&nums, Some(i)))
        })
        .count();

    println!("{:?}", ans);
}
