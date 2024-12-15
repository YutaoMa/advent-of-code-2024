use std::fs;

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    let mut parts = line.split(" ");
    let part_p = parts.next().unwrap();
    let p = part_p.strip_prefix("p=").unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let part_v = parts.next().unwrap();
    let v = part_v.strip_prefix("v=").unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    
    ((p[0], p[1]), (v[0], v[1]))
}

fn update_position(p: (i32, i32), v: (i32, i32), n: i32, m: i32, rounds: i32) -> (i32, i32) {
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

    // Control the number of rounds by updating the range in the for loop
    for i in 7093..7097 {
        print!("\x1B[2J\x1B[1;1H");

        println!("Round: {}", i);

        let robots = input.lines()
            .map(|line| parse_line(line))
            .map(|(p, v)| update_position(p, v, n, m, i))
            .collect::<Vec<(i32, i32)>>();

        let mut grid = vec![vec![0; n as usize]; m as usize];
        for robot in robots {
            grid[robot.1 as usize][robot.0 as usize] += 1;
        }

        for row in grid.iter() {
            for cell in row.iter() {
                if *cell > 0 {
                    print!("{}", cell);
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        // Control the speed of the animation by updating the duration in the sleep function
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

}
