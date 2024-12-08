use std::{collections::{HashMap, HashSet}, fs};

pub fn main() {
    let input = fs::read_to_string("input/8.txt").expect("Unable to read input file");

    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut seen: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (y, row) in matrix.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '.' {
                continue;
            }

            let entry = seen.entry(*c).or_insert(Vec::new());

            for &(x2, y2) in entry.iter() {
                let x = x as i32;
                let y = y as i32;
                let x2 = x2 as i32;
                let y2 = y2 as i32;

                for (ant_x, ant_y) in [
                    (x2 - (x - x2), y2 - (y - y2)),
                    (x + (x - x2), y + (y - y2)),
                ] {
                    if ant_x >= 0 && ant_x < matrix[0].len() as i32 && ant_y >= 0 && ant_y < matrix.len() as i32 {
                        antinodes.insert((ant_x, ant_y));
                    }
                }
            }

            entry.push((x, y));
        }
    }

    dbg!(antinodes.len());
}
