use std::{collections::HashSet, fs};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_coords(&self, i: i32, j: i32) -> (i32, i32) {
        match self {
            Direction::Up => (i - 1, j),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn is_loop(matrix: &[Vec<char>], start_i: i32, start_j: i32) -> bool {
    let mut visited: HashSet<(i32, i32, Direction)> = HashSet::new();

    let mut i = start_i;
    let mut j = start_j;
    let mut direction = Direction::Up;


    loop {
        if visited.contains(&(i, j, direction)) {
            return true;
        }

        visited.insert((i, j, direction));

        let (next_i, next_j) = direction.move_coords(i, j);
        if next_i >= 0 && next_i < matrix.len() as i32 && next_j >= 0 && next_j < matrix[0].len() as i32 {
            if matrix[next_i as usize][next_j as usize] == '#' {
                direction = direction.turn_right();
            } else {
                i = next_i;
                j = next_j;
            }
        } else {
            return false;
        }
    }
}

pub fn main() {
    let input = fs::read_to_string("input/6.txt").expect("Unable to read input file");

    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (start_i, start_j) = matrix.iter().enumerate().find_map(|(i, row)| {
        row.iter().enumerate().find_map(|(j, &c)| if c == '^' { Some((i, j)) } else { None })
    }).unwrap();

    let mut ans = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '.' {
                matrix[i][j] = '#';
                if is_loop(&matrix, start_i as i32, start_j as i32) {
                    ans += 1;
                }
                matrix[i][j] = '.';
            }
        }
    }

    dbg!(ans);
}
