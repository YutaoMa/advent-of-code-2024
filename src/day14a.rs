use std::fs;

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    let mut parts = line.split(" ");
    let part_p = parts.next().unwrap();
    let p = part_p.strip_prefix("p=").unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let part_v = parts.next().unwrap();
    let v = part_v.strip_prefix("v=").unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    
    ((p[0], p[1]), (v[0], v[1]))
}

fn update_position(p: (i32, i32), v: (i32, i32), n: i32, m: i32) -> (i32, i32) {
    let rounds = 100;
    let mut new_x = (p.0 + v.0 * rounds) % n;
    let mut new_y = (p.1 + v.1 * rounds) % m;
    if new_x < 0 {
        new_x += n;
    }

    if new_y < 0 {
        new_y += m;
    }

    (new_x, new_y)
}

pub fn main() {
    let input = fs::read_to_string("input/14.txt").expect("Unable to read input file");

    let m = 103;
    let n = 101;

    let robots = input.lines()
        .map(|line| parse_line(line))
        .map(|(p, v)| update_position(p, v, n, m))
        .collect::<Vec<(i32, i32)>>();

    let mut count = vec![0; 4];

    for (j, i) in robots {
        if i >= 0 && i < m / 2 {
            if j >= 0 && j < n / 2 {
                count[0] += 1;
            } else if j > n / 2 && j < n {
                count[1] += 1;
            }
        } else if i > m / 2 && i < m {
            if j >= 0 && j < n / 2 {
                count[3] += 1;
            } else if j > n / 2 && j < n {
                count[2] += 1;
            }
        }
    }

    dbg!(count.iter().product::<i32>());
}
