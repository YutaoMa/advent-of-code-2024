use std::fs;

fn parse_line(line: &str, prefix: &str) -> (i32, i32) {
    let parts = line.strip_prefix(prefix)
        .unwrap()
        .split(", ")
        .map(|s| s.split_at(2).1.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    (parts[0], parts[1])
}

fn parse_section(input: &str) -> ((i32, i32), (i32, i32), (i32, i32)) {
    let mut lines = input.lines();

    let button_a = parse_line(lines.next().unwrap(), "Button A: ");
    let button_b = parse_line(lines.next().unwrap(), "Button B: ");
    let prize = parse_line(lines.next().unwrap(), "Prize: ");

    (button_a, button_b, prize)
}

fn min_cost(a: (i32, i32), b: (i32, i32), prize: (i32, i32)) -> i32 {
    let mut min_cost = i32::MAX;

    let max_a = (prize.0 / a.0).min(prize.1 / a.1).min(100);

    for i in 0..=max_a {
        let prize_remain = (prize.0 - a.0 * i, prize.1 - a.1 * i);
        if prize_remain.0 % b.0 == 0 && prize_remain.0 / b.0 == prize_remain.1 / b.1 {
            let cost = 3 * i + prize_remain.0 / b.0;
            min_cost = min_cost.min(cost);
        }
    }

    min_cost
}

pub fn main() {
    let input = fs::read_to_string("input/13.txt").expect("Unable to read input file");

    let total_cost: i32 = input
        .split("\n\n")
        .map(parse_section)
        .map(|(a, b, prize)| min_cost(a, b, prize))
        .filter(|&cost| cost != i32::MAX)
        .sum();

    dbg!(total_cost);
}
