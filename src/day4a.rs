use std::fs;

fn transpose(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    let rows = matrix.len();
    if rows == 0 {
        return Vec::new();
    }

    let cols = matrix[0].len();
    (0..cols)
        .map(|j| (0..rows).map(|i| matrix[i][j]).collect())
        .collect()
}

fn collect_diagonals(matrix: &[Vec<char>]) -> (Vec<String>, Vec<String>) {
    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };

    let mut primary_diagonals: Vec<String> = Vec::new();
    let mut secondary_diagonals: Vec<String> = Vec::new();

    for start in 0..rows + cols - 1 {
        let primary: String = (0..=start)
            .filter_map(|i| {
                let j = start - i;
                if i < rows && j < cols {
                    Some(matrix[i][j])
                } else {
                    None
                }
            })
            .collect();

        if !primary.is_empty() {
            primary_diagonals.push(primary);
        }

        let secondary: String = (0..=start)
            .filter_map(|i| {
                let j = start - i;
                if i < rows && j < cols {
                    Some(matrix[i][cols - 1 - j])
                } else {
                    None
                }
            })
            .collect();

        if !secondary.is_empty() {
            secondary_diagonals.push(secondary);
        }
    }

    (primary_diagonals, secondary_diagonals)
}

fn count_patterns(lines: impl Iterator<Item = String>, patterns: &[&str]) -> usize {
    lines
        .map(|line| {
            patterns
                .iter()
                .map(|pattern| line.matches(pattern).count())
                .sum::<usize>()
        })
        .sum()
}

pub fn main() {
    let input = fs::read_to_string("input/4.txt").expect("Unable to read input file");

    let patterns = ["XMAS", "SAMX"];

    let mut ans = count_patterns(input.lines().map(String::from), &patterns);

    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let transposed = transpose(&matrix);
    ans += count_patterns(transposed.iter().map(|row| row.iter().collect()), &patterns);

    let (primary_diagonals, secondary_diagonals) = collect_diagonals(&matrix);
    ans += count_patterns(primary_diagonals.into_iter(), &patterns);
    ans += count_patterns(secondary_diagonals.into_iter(), &patterns);

    dbg!(ans);
}
