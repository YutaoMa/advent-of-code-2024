use std::{collections::VecDeque, fs};

fn bfs(maze: &[Vec<char>]) -> Option<usize> {
    if maze.is_empty() || maze[0].is_empty() {
        return None;
    }

    let rows = maze.len();
    let cols = maze[0].len();

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    if maze[0][0] == '#' || maze[rows - 1][cols - 1] == '#' {
        return None;
    }

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));

    let mut visited = vec![vec![false; cols]; rows];
    visited[0][0] = true;

    while let Some((row, col, dist)) = queue.pop_front() {
        if row == rows - 1 && col == cols - 1 {
            return Some(dist);
        }

        for (dr, dc) in directions.iter() {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row >= 0
            && new_row < rows as isize
            && new_col >= 0
            && new_col < cols as isize
            && maze[new_row as usize][new_col as usize] == '.'
            && !visited[new_row as usize][new_col as usize]
            {
                visited[new_row as usize][new_col as usize] = true;
                queue.push_back((new_row as usize, new_col as usize, dist + 1));
            }
        }
    }

    None
}

pub fn main() {
    let input = fs::read_to_string("input/18.txt").expect("Unable to read input file");

    let m = 71;
    let n = 71;

    let mut maze: Vec<Vec<char>> = vec![vec!['.'; n]; m];

    let lines: Vec<&str> = input.lines().collect();

    for i in 0..1024 {
        let mut fields = lines[i].split(',');
        let j = fields.next().unwrap().parse::<usize>().unwrap();
        let i = fields.next().unwrap().parse::<usize>().unwrap();

        maze[i][j] = '#';
    }

    let result = bfs(&maze);

    match result {
        Some(dist) => println!("{}", dist),
        None => println!("No path found"),
    }
}
