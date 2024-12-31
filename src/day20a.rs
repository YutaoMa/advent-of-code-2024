use std::collections::VecDeque;
use std::fs;

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const JUMP_DIRECTIONS: [(isize, isize); 4] = [(0, 2), (2, 0), (0, -2), (-2, 0)];

fn bfs(maze: &[Vec<char>], start_i: usize, start_j: usize) -> Option<usize> {
    if maze.is_empty() || maze[0].is_empty() {
        return None;
    }

    let rows = maze.len();
    let cols = maze[0].len();
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; cols]; rows];

    queue.push_back((start_i, start_j, 0));
    visited[start_i][start_j] = true;

    while let Some((row, col, dist)) = queue.pop_front() {
        if maze[row][col] == 'E' {
            return Some(dist);
        }

        for &(dr, dc) in &DIRECTIONS {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if (0..rows as isize).contains(&new_row)
                && (0..cols as isize).contains(&new_col)
            {
                let (nr, nc) = (new_row as usize, new_col as usize);

                if !visited[nr][nc] && matches!(maze[nr][nc], '.' | 'E') {
                    visited[nr][nc] = true;
                    queue.push_back((nr, nc, dist + 1));
                }
            }
        }
    }

    None
}

pub fn main() {
    let input = fs::read_to_string("input/20.txt").expect("Unable to read input file");

    let maze: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut distance_maze: Vec<Vec<Option<usize>>> = maze
        .iter()
        .map(|row| row.iter().map(|_| None).collect())
        .collect();

    for (i, row) in maze.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if matches!(cell, '.' | 'S' | 'E') {
                distance_maze[i][j] = bfs(&maze, i, j);
            }
        }
    }

    let mut count = 0;
    let save = 100;

    for (i, row) in distance_maze.iter().enumerate() {
        for (j, &dist) in row.iter().enumerate() {
            if let Some(dist) = dist {
                for &(dr, dc) in &JUMP_DIRECTIONS {
                    let new_i = i as isize + dr;
                    let new_j = j as isize + dc;

                    if (0..maze.len() as isize).contains(&new_i)
                        && (0..maze[0].len() as isize).contains(&new_j)
                    {
                        let (ni, nj) = (new_i as usize, new_j as usize);

                        if let Some(adj_dist) = distance_maze[ni][nj] {
                            if adj_dist + 2 + save <= dist {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}
