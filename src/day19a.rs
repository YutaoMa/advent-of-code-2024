use std::fs;
use std::collections::HashMap;

fn can_be_constructed(parts: &Vec<&str>, goal: &str, memo: &mut HashMap<String, bool>) -> bool {
    if let Some(&result) = memo.get(goal) {
        return result;
    }

    if parts.contains(&goal) {
        memo.insert(goal.to_string(), true);
        return true;
    }

    for part in parts {
        if goal.starts_with(part) {
            let remaining = &goal[part.len()..];
            if can_be_constructed(parts, remaining, memo) {
                memo.insert(goal.to_string(), true);
                return true;
            }
        }
    }

    memo.insert(goal.to_string(), false);
    false
}

pub fn main() {
    let input = fs::read_to_string("input/19.txt").expect("Unable to read input file");

    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let parts = sections[0].split(", ").collect::<Vec<&str>>();
    let goals = sections[1].lines().collect::<Vec<&str>>();

    let mut memo = HashMap::new();

    let ans = goals.iter()
        .filter(|&&goal| can_be_constructed(&parts, goal, &mut memo))
        .count();

    println!("{}", ans);
}
