use std::fs;
use std::collections::HashSet;

fn dfs(matrix: &[Vec<u32>], seen: &mut HashSet<(usize, usize)>, i: usize, j: usize) -> u32 {
    if !seen.insert((i, j)) {
        return 0;
    }

    let curr = matrix[i][j];

    if curr == 9 {
        return 1;
    }

    let mut ans = 0;
    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    for &(di, dj) in &directions {
        let new_i = i as i32 + di;
        let new_j = j as i32 + dj;

        if new_i >= 0 && new_i < matrix.len() as i32 && new_j >= 0 && new_j < matrix[0].len() as i32 {
            let new_i = new_i as usize;
            let new_j = new_j as usize;

            if matrix[new_i][new_j] == curr + 1 {
                ans += dfs(matrix, seen, new_i, new_j);
            }
        }
    }

    ans
}

pub fn main() {
    let input = fs::read_to_string("input/10.txt").expect("Unable to read input file");

    let matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 0 {
                ans += dfs(&matrix, &mut HashSet::new(), i, j);
            }
        }
    }

    dbg!(ans);
}
