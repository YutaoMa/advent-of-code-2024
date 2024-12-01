use std::fs;
use std::collections::HashMap;

pub fn main() {
    let input = fs::read_to_string("input/1.txt").expect("Unable to read input file");

    let (nums, freq): (Vec<i32>, HashMap<i32, i32>) = input
        .lines()
        .map(|line| {
            let mut fields = line.split_whitespace();
            let num1 = fields.next().expect("Missing num1").parse::<i32>().expect("Invalid num1");
            let num2 = fields.next().expect("Missing num2").parse::<i32>().expect("Invalid num2");
            (num1, num2)
        })
        .fold((Vec::new(), HashMap::new()), |(mut nums, mut freq), (num1, num2)| {
            nums.push(num1);
            *freq.entry(num2).or_insert(0) += 1;
            (nums, freq)
        });

    let ans = nums.iter().map(|num| {
        num * freq.get(num).unwrap_or(&0)
    }).sum::<i32>();

    println!("{:?}", ans);
}
