use std::fs;

pub fn main() {
    let input = fs::read_to_string("input/1.txt").expect("Unable to read input file");

    let (mut nums1, mut nums2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut fields = line.split_whitespace();
            let num1 = fields.next().expect("Missing num1").parse::<i32>().expect("Invalid num1");
            let num2 = fields.next().expect("Missing num2").parse::<i32>().expect("Invalid num2");
            (num1, num2)
        })
        .unzip();

    nums1.sort();
    nums2.sort();

    let ans = nums1.iter().zip(&nums2).map(|(a, b)| (a - b).abs()).sum::<i32>();

    println!("{:?}", ans);
}
