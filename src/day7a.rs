use std::fs;

fn is_possible(nums: &[u64], index: usize, curr: u64, target: u64) -> bool {
    if index == nums.len() {
        return curr == target;
    }

    is_possible(nums, index + 1, curr + nums[index], target) || is_possible(nums, index + 1, curr * nums[index], target)
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let mut fields = line.split(": ");
    let target = fields.next().unwrap().parse::<u64>().unwrap();
    let nums = fields.next().unwrap().split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
    (target, nums)
}

pub fn main() {
    let input = fs::read_to_string("input/7.txt").expect("Unable to read input file");

    let sum: u64 = input
        .lines()
        .map(parse_line)
        .filter(|(target, nums)| is_possible(&nums, 1, nums[0], *target))
        .map(|(target, _)| target)
        .sum();

    dbg!(sum);
}
