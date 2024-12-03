use std::fs;
use regex::Regex;

fn sanitize(input: &str) -> String {
    let start_pattern = "don't()";
    let end_pattern = "do()";
    let mut result = input.to_string();

    while let Some(start_index) = result.find(start_pattern) {
        if let Some(end_index) = result[start_index..].find(end_pattern) {
            let end_index = start_index + end_index + end_pattern.len();
            result.replace_range(start_index..end_index, "");
        } else {
            result.truncate(start_index);
            break;
        }
    }

    result
}

pub fn main() {
    let input = fs::read_to_string("input/3.txt").expect("Unable to read input file");

    let re = Regex::new("mul\\((\\d+),(\\d+)\\)").unwrap();

    let input = sanitize(&input);

    let ans = re.captures_iter(&input).map(|cap| {
        let a: i32 = cap[1].parse().unwrap();
        let b: i32 = cap[2].parse().unwrap();
        a * b
    }).sum::<i32>();

    dbg!(ans);
}
