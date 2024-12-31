use std::fs;

fn parse_schematic(schematic: &[Vec<char>]) -> Vec<u32> {
    let mut result = vec![0; schematic[0].len()];
    match schematic[0][0] {
        '#' => {
            // Lock: Count '#' from top to bottom
            for (j, column) in result.iter_mut().enumerate() {
                for row in schematic {
                    if row[j] == '#' {
                        *column += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        '.' => {
            // Key: Count '#' from bottom to top
            for (j, column) in result.iter_mut().enumerate() {
                for row in schematic.iter().rev() {
                    if row[j] == '#' {
                        *column += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        _ => panic!("Invalid schematic: unknown upper-left corner character"),
    }
    result
}

pub fn main() {
    let input = fs::read_to_string("input/25.txt").expect("Unable to read input file");

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for section in input.split("\n\n") {
        let schematic: Vec<Vec<char>> = section.lines().map(|line| line.chars().collect()).collect();

        match schematic.get(0).and_then(|row| row.get(0)) {
            Some('#') => locks.push(parse_schematic(&schematic)),
            Some('.') => keys.push(parse_schematic(&schematic)),
            _ => panic!("Invalid schematic: unknown upper-left corner character"),
        }
    }

    let height = 7;
    let ans = locks.iter().flat_map(|lock| {
        keys.iter().filter(move |key| {
            lock.iter()
                .zip(key.iter())
                .all(|(&lock_val, &key_val)| lock_val + key_val <= height)
        })
    }).count();

    dbg!(ans);
}
