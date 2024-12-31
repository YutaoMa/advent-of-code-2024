use std::collections::{HashMap, HashSet};
use std::fs;

fn largest_fully_connected_group<'a>(
    network_map: &'a HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<&'a str> {
    let mut largest_clique = HashSet::new();
    let mut r = HashSet::new();
    let mut p: HashSet<&str> = network_map.keys().copied().collect();
    let mut x = HashSet::new();

    bron_kerbosch(network_map, &mut r, &mut p, &mut x, &mut largest_clique);

    largest_clique
}

fn bron_kerbosch<'a>(
    network_map: &'a HashMap<&'a str, HashSet<&'a str>>,
    r: &mut HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    largest_clique: &mut HashSet<&'a str>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > largest_clique.len() {
            *largest_clique = r.clone();
        }
        return;
    }

    let pivot = p.union(x).next().copied();

    let candidates = match pivot {
        Some(pivot) => p
            .difference(network_map.get(pivot).unwrap_or(&HashSet::new()))
            .copied()
            .collect::<Vec<_>>(),
        None => p.iter().copied().collect::<Vec<_>>(),
    };

    for &node in &candidates {
        r.insert(node);

        let empty: HashSet<&str> = HashSet::new();
        let neighbors = network_map.get(node).unwrap_or(&empty);
        let new_p: HashSet<&str> = p.intersection(neighbors).copied().collect();
        let new_x: HashSet<&str> = x.intersection(neighbors).copied().collect();

        bron_kerbosch(
            network_map,
            r,
            &mut new_p.clone(),
            &mut new_x.clone(),
            largest_clique,
        );

        r.remove(node);
        p.remove(node);
        x.insert(node);
    }
}

pub fn main() {
    let input = fs::read_to_string("input/23.txt").expect("Unable to read input file");

    let mut network_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let (key, value) = line
            .split_once('-')
            .expect("Each line must have a '-' separator");
        network_map.entry(key).or_default().insert(value);
        network_map.entry(value).or_default().insert(key);
    }

    let ans = largest_fully_connected_group(&network_map);

    let mut password: Vec<&str> = ans.into_iter().collect();
    password.sort();

    println!("{}", password.join(","));
}
