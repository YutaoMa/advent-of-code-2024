use std::fs;
use regex::Regex;

pub fn main() {
    let input = fs::read_to_string("input/3.txt").expect("Unable to read input file");

    let re = Regex::new("mul\\((\\d+),(\\d+)\\)").unwrap();

    let ans = re.captures_iter(&input).map(|cap| {
        let a: i32 = cap[1].parse().unwrap();
        let b: i32 = cap[2].parse().unwrap();
        a * b
    }).sum::<i32>();

    dbg!(ans);
}
