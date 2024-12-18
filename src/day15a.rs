use std::fs;

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!("Invalid move"),
        }
    }
}

impl Move {
    fn to_dir(&self) -> (i32, i32) {
        match self {
            Move::Up => (-1, 0),
            Move::Down => (1, 0),
            Move::Left => (0, -1),
            Move::Right => (0, 1),
        }
    }
}

fn try_move(matrix: &mut Vec<Vec<char>>, i: usize, j: usize, m: &Move) -> bool {
    let c = matrix[i][j];

    if c == '#' {
        return false;
    }

    if c == '.' {
        return true;
    }

    let (di, dj) = m.to_dir();
    let new_i = i as i32 + di;
    let new_j = j as i32 + dj;

    if try_move(matrix, new_i as usize, new_j as usize, m) {
        matrix[new_i as usize][new_j as usize] = c;
        matrix[i][j] = '.';
        return true;
    } else {
        return false;
    }
}

pub fn main() {
    let input = fs::read_to_string("input/15.txt").expect("Unable to read input file");

    let mut sections = input.split("\n\n");

    let mut matrix: Vec<Vec<char>> = sections.next().unwrap().lines().map(|line| line.chars().collect()).collect();

    let moves: Vec<Move> = sections.next().unwrap().lines().flat_map(|line| line.chars().map(Move::from)).collect();

    let (mut robot_i, mut robot_j) = matrix.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, c)| {
                    if *c == '@' {
                        Some((i, j))
                    } else {
                        None
                    }
                })
        })
        .next()
        .unwrap();


    for m in moves {
        if try_move(&mut matrix, robot_i, robot_j, &m) {
            let (di, dj) = m.to_dir();
            robot_i = (robot_i as i32 + di) as usize;
            robot_j = (robot_j as i32 + dj) as usize;
        }
    }

    let mut ans = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'O' {
                ans += (100 * i) + j;
            }
        }
    }

    println!("{}", ans);
}
