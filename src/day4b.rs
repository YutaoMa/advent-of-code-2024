use std::fs;

pub fn main() {
    let input = fs::read_to_string("input/4.txt").expect("Unable to read input file");

    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = matrix.len();
    let cols = matrix[0].len();

    let ans: i32 = (1..rows-1)
        .flat_map(|i| {
            (1..cols-1).filter_map(|j| {
                if matrix[i][j] == 'A'
                && matches!((matrix[i-1][j-1], matrix[i+1][j+1]), ('M', 'S') | ('S', 'M'))
                && matches!((matrix[i-1][j+1], matrix[i+1][j-1]), ('M', 'S') | ('S', 'M'))
                {
                    Some(1)
                } else {
                    None
                }
            }).collect::<Vec<i32>>()
        })
        .sum();

    dbg!(ans);
}
