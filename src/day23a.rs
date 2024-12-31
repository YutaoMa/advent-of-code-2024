use std::collections::{HashMap, HashSet};
use std::fs;

pub fn main() {
    let input = fs::read_to_string("input/23.txt").expect("Unable to read input file");

    let mut network_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (key, value) = line
            .split_once('-')
            .expect("Each line must have a '-' separator");
        network_map.entry(key).or_default().push(value);
        network_map.entry(value).or_default().push(key);
    }

    let mut sets: HashSet<Vec<&str>> = HashSet::new();

    for (key, neighbors) in &network_map {
        if !key.starts_with('t') || neighbors.len() < 2 {
            continue;
        }

        for (i, a) in neighbors.iter().enumerate() {
            for b in neighbors.iter().skip(i + 1) {
                if let Some(connections) = network_map.get(a) {
                    if connections.contains(b) {
                        let mut triplet = vec![*key, *a, *b];
                        triplet.sort();
                        sets.insert(triplet);
                    }
                }
            }
        }
    }

    println!("Number of unique sets: {}", sets.len());
}
