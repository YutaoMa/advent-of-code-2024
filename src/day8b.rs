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

                let dx = x - x2;
                let dy = y - y2;

                for &(mut ant_x, mut ant_y, step_x, step_y) in &[
                    (x2 - dx, y2 - dy, -dx, -dy),
                    (x + dx, y + dy, dx, dy),
                ] {
                    while ant_x >= 0 && ant_x < matrix[0].len() as i32 && ant_y >= 0 && ant_y < matrix.len() as i32 {
                        antinodes.insert((ant_x, ant_y));

                        ant_x += step_x;
                        ant_y += step_y;
                    }
                }
            }

            entry.push((x, y));
        }
    }

    seen.values()
        .filter(|v| v.len() >= 2)
        .flat_map(|v| v.iter().map(|(x, y)| (*x as i32, *y as i32)))
        .for_each(|(x, y)| {
            antinodes.insert((x, y));
        });

    dbg!(antinodes.len());
}
