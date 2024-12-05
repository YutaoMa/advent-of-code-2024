use std::{collections::{HashMap, HashSet}, fs};

fn is_order_valid(update: &str, rules_map: &HashMap<&str, HashSet<&str>>) -> bool {
    let mut seen = HashSet::new();

    for page in update.split(',') {
        if let Some(posts) = rules_map.get(page) {
            if posts.iter().any(|post| seen.contains(post)) {
                return false;
            }
        }
        seen.insert(page);
    }

    true
}

pub fn main() {
    let input = fs::read_to_string("input/5.txt").expect("Unable to read input file");

    let mut sections = input.split("\n\n");

    let rules = sections.next().unwrap();
    let updates = sections.next().unwrap();

    let rules_map: HashMap<&str, HashSet<&str>> = rules
        .lines()
        .map(|rule| {
            let mut parts = rule.split("|");
            let pre = parts.next().unwrap();
            let post = parts.next().unwrap();
            (pre, post)
        })
        .fold(HashMap::new(), |mut map, (pre, post)| {
            map.entry(pre).or_insert_with(HashSet::new).insert(post);
            map
        });

    let ans: i32 = updates
        .lines()
        .filter(|update| is_order_valid(update, &rules_map))
        .map(|update| {
            let pages = update.split(',').collect::<Vec<&str>>();
            let mid_page = pages.get(pages.len() / 2).unwrap();
            mid_page.parse::<i32>().unwrap()
        })
        .sum();

    dbg!(ans);
}
