use std::fs;

// -> (area, perimeter)
fn dfs(matrix: &[Vec<char>], i: usize, j: usize, visited: &mut Vec<Vec<bool>>) -> (i32, i32) {
    if visited[i][j] {
        return (0, 0);
    }

    visited[i][j] = true;

    let mut area = 1;
    let mut perimeter = 0;
    let c = matrix[i][j];

    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (di, dj) in dirs.iter() {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;

        if ni >= 0 && ni < matrix.len() as i32 && nj >= 0 && nj < matrix[0].len() as i32 && matrix[ni as usize][nj as usize] == c {
            let (a, p) = dfs(matrix, ni as usize, nj as usize, visited);
            area += a;
            perimeter += p;
        } else {
            perimeter += 1;
        }
    }

    (area, perimeter)
}

pub fn main() {
    let input = fs::read_to_string("input/12.txt").expect("Unable to read input file");

    let matrix: Vec<Vec<char>> = input
        .trim()
        .split_whitespace()
        .map(|x| x.chars().collect())
        .collect();

    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];

    let mut ans = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if !visited[i][j] {
                let (area, perimeter) = dfs(&matrix, i, j, &mut visited);
                ans += area * perimeter;
            }
        }
    }

    dbg!(ans);
}
