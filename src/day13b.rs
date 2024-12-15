use std::fs;

fn parse_line(line: &str, prefix: &str) -> (i64, i64) {
    let parts: Vec<i64> = line
        .strip_prefix(prefix)
        .unwrap()
        .split(", ")
        .map(|s| s.split_at(2).1.parse::<i64>().unwrap())
        .collect();
    (parts[0], parts[1])
}

fn parse_section(input: &str) -> ((i64, i64), (i64, i64), (i64, i64)) {
    let mut lines = input.lines();
    let button_a = parse_line(lines.next().unwrap(), "Button A: ");
    let button_b = parse_line(lines.next().unwrap(), "Button B: ");
    let prize = parse_line(lines.next().unwrap(), "Prize: ");
    (button_a, button_b, prize)
}

fn min_cost(a: (i64, i64), b: (i64, i64), p: (i64, i64)) -> i64 {
    let det = a.0 * b.1 - a.1 * b.0;
    if det == 0 {
        return 0;
    }

    let mut a_step = b.1 * p.0 - b.0 * p.1;
    let mut b_step = a.0 * p.1 - a.1 * p.0;

    if a_step % det != 0 || b_step % det != 0 {
        return 0;
    }

    a_step /= det;
    b_step /= det;

    3 * a_step + b_step
}

pub fn main() {
    let input = fs::read_to_string("input/13.txt").expect("Unable to read input file");

    let additional_prize = 10_000_000_000_000;

    let total_cost: i64 = input
        .split("\n\n")
        .map(parse_section)
        .map(|(a, b, prize)| {
            min_cost(a, b, (prize.0 + additional_prize, prize.1 + additional_prize))
        })
        .sum();

    dbg!(total_cost);
}
