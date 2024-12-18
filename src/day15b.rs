use std::fs;

#[derive(Debug, Eq, PartialEq)]
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

fn can_move(matrix: &Vec<Vec<char>>, i: usize, j: usize, m: &Move, is_box: bool) -> bool {
    let c = matrix[i][j];

    match c {
        '#' => false,
        '.' => true,
        '[' | ']' => match m {
            Move::Up | Move::Down => {
                let (di, _) = m.to_dir();
                let next_i = (i as i32 + di) as usize;
                can_move(matrix, next_i, j, m, false)
                    && (is_box || can_move(matrix, i, if c == '[' { j + 1 } else { j - 1 }, m, true))
            },
            Move::Left | Move::Right => {
                let (_, dj) = m.to_dir();
                let next_j = (j as i32 + dj) as usize;
                can_move(matrix, i, next_j, m, false)
            },
        },
        '@' => {
            let (di, dj) = m.to_dir();
            let new_i = (i as i32 + di) as usize;
            let new_j = (j as i32 + dj) as usize;
            can_move(matrix, new_i, new_j, m, false)
        },
        _ => panic!("Invalid character"),
    }
}

fn execute_move(matrix: &mut Vec<Vec<char>>, i: usize, j: usize, m: &Move, is_box: bool) {
    let c = matrix[i][j];

    if c == '#' {
        panic!("Invalid move");
    }

    if c == '.' {
        return;
    }

    let (di, dj) = m.to_dir();
    let new_i = (i as i32 + di) as usize;
    let new_j = (j as i32 + dj) as usize;

    match c {
        '[' | ']' => {
            execute_move(matrix, new_i, new_j, m, false);
            if (*m == Move::Up || *m == Move::Down) && !is_box {
                execute_move(matrix, i, if c == '[' { j + 1 } else { j - 1 }, m, true);
            }
        },
        _ => {
            execute_move(matrix, new_i, new_j, m, false);
        }
    }

    matrix[new_i][new_j] = c;
    matrix[i][j] = '.';
}

fn resize_matrix(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    matrix
        .into_iter()
        .map(|row| {
            row.into_iter()
                .flat_map(|c| {
                    match c {
                        '@' => vec!['@', '.'],
                        'O' => vec!['[', ']'],
                        c => vec![c, c],
                    }
                })
                .collect()
        }).collect()
}

pub fn main() {
    let input = fs::read_to_string("input/15.txt").expect("Unable to read input file");

    let mut sections = input.split("\n\n");

    let matrix: Vec<Vec<char>> = sections.next().unwrap().lines().map(|line| line.chars().collect()).collect();
    
    let mut matrix = resize_matrix(matrix);

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
        if can_move(&matrix, robot_i, robot_j, &m, false) {
            execute_move(&mut matrix, robot_i, robot_j, &m, false);
            let (di, dj) = m.to_dir();
            robot_i = (robot_i as i32 + di) as usize;
            robot_j = (robot_j as i32 + dj) as usize;
        }
    }

    let mut ans = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '[' {
                ans += (100 * i) + j;
            }
        }
    }

    println!("{}", ans);
}
