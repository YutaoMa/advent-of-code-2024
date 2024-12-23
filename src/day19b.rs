use std::fs;
use std::collections::HashMap;

fn count_ways(parts: &Vec<&str>, goal: &str, memo: &mut HashMap<String, usize>) -> usize {
    if let Some(&result) = memo.get(goal) {
        return result;
    }

    if goal.is_empty() {
        return 1;
    }

    let mut ways = 0;

    for part in parts {
        if goal.starts_with(part) {
            let remaining = &goal[part.len()..];
            ways += count_ways(parts, remaining, memo);
        }
    }

    memo.insert(goal.to_string(), ways);
    ways
}

pub fn main() {
    let input = fs::read_to_string("input/19.txt").expect("Unable to read input file");

    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let parts = sections[0].split(", ").collect::<Vec<&str>>();
    let goals = sections[1].lines().collect::<Vec<&str>>();

    let mut memo = HashMap::new();

    let mut total_ways = 0;

    for &goal in &goals {
        let ways = count_ways(&parts, goal, &mut memo);
        total_ways += ways;
    }

    println!("Total Unique Ways: {}", total_ways);
}
