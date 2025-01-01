use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;
use std::iter;

const NUMERIC: [&str; 4] = ["789", "456", "123", " 0A"];
const DIRECTIONAL: [&str; 2] = [" ^A", "<v>"];
const DIRECTIONS: [char; 4] = ['<', '>', '^', 'v'];

fn walk(keypad: &[&str], mut x: usize, mut y: usize, path: &str) -> Vec<char> {
    let mut result = Vec::new();
    for direction in path.chars() {
        let neighbors = [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ];
        if let Some(index) = DIRECTIONS.iter().position(|&d| d == direction) {
            let (new_x, new_y) = neighbors[index];
            if new_y < keypad.len() && new_x < keypad[new_y].len() {
                x = new_x;
                y = new_y;
                result.push(keypad[y].chars().nth(x).unwrap());
            }
        }
    }
    result
}

fn find_key(keypad: &[&str], key: char) -> Option<(usize, usize)> {
    for (y, row) in keypad.iter().enumerate() {
        if let Some(x) = row.chars().position(|k| k == key) {
            return Some((x, y));
        }
    }
    None
}

fn paths_between(keypad: &[&str], start: char, end: char) -> Vec<String> {
    let (x1, y1) = find_key(keypad, start).expect("Start key not found");
    let (x2, y2) = find_key(keypad, end).expect("End key not found");

    let hor = if x2 > x1 { ">".repeat(x2 - x1) } else { "<".repeat(x1 - x2) };
    let ver = if y2 > y1 { "v".repeat(y2 - y1) } else { "^".repeat(y1 - y2) };

    let mut paths = HashSet::new();
    paths.insert(format!("{}{}A", hor, ver));
    paths.insert(format!("{}{}A", ver, hor));

    paths
        .into_iter()
        .filter(|path| !walk(keypad, x1, y1, path).contains(&' '))
        .collect()
}

fn cost_between(
    keypad: &[&str],
    start: char,
    end: char,
    links: usize,
    memo: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    if links == 0 {
        return 1;
    }

    if let Some(&cached) = memo.get(&(start, end, links)) {
        return cached;
    }

    let result = paths_between(keypad, start, end)
        .iter()
        .map(|path| cost(&DIRECTIONAL, path, links - 1, memo))
        .min()
        .unwrap_or(usize::MAX);

    memo.insert((start, end, links), result);
    result
}

fn cost(
    keypad: &[&str],
    keys: &str,
    links: usize,
    memo: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    let mut total_cost = 0;
    let mut chars = iter::once('A').chain(keys.chars()).peekable();

    while let Some(a) = chars.next() {
        if let Some(&b) = chars.peek() {
            total_cost += cost_between(keypad, a, b, links, memo);
        }
    }

    total_cost
}

fn complexity(code: &str, robots: usize) -> usize {
    let mut memo = HashMap::new();
    cost(&NUMERIC, code, robots + 1, &mut memo) * code[..code.len() - 1]
        .parse::<usize>()
        .unwrap_or(0)
}

pub fn main() {
    let input = fs::read_to_string("input/21.txt").expect("Unable to read input file");

    let codes: Vec<&str> = input.lines().collect();

    let ans: usize = codes.iter().map(|&code| complexity(code, 2)).sum();

    println!("{ans}");
}
