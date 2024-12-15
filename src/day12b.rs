use std::fs;

use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn iter() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }

    fn next(self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Self::Up if x > 0 => Some((x - 1, y)),
            Self::Down => Some((x + 1, y)),
            Self::Left if y > 0 => Some((x, y - 1)),
            Self::Right => Some((x, y + 1)),
            _ => None,
        }
    }

    fn right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

fn flood_fill(
    matrix: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
    start: (usize, usize),
) -> Option<(HashSet<(usize, usize)>, usize)> {
    if visited.contains(&start) || matrix[start.0][start.1] == ' ' {
        return None;
    }

    let mut stack = vec![start];
    let mut area = HashSet::new();
    let mut perimeter = 0;
    let target = matrix[start.0][start.1];

    while let Some((x, y)) = stack.pop() {
        if !visited.insert((x, y)) {
            continue;
        }
        area.insert((x, y));

        for dir in Direction::iter() {
            if let Some((nx, ny)) = dir.next((x, y)) {
                if nx < matrix.len() && ny < matrix[0].len() && matrix[nx][ny] == target {
                    stack.push((nx, ny));
                } else {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    Some((area, perimeter))
}

fn calculate_corners(area: &HashSet<(usize, usize)>) -> usize {
    Direction::iter()
        .iter()
        .flat_map(|dir| {
            area.iter().flat_map(move |&point| {
                let next = dir.next(point);
                let right_next = dir.right().next(point);
                let corner = next.and_then(|p| dir.right().next(p));
                [
                    next.map_or(false, |p| area.contains(&p))
                        && right_next.map_or(false, |p| area.contains(&p))
                        && corner.map_or(true, |p| !area.contains(&p)),
                    next.map_or(true, |p| !area.contains(&p))
                        && right_next.map_or(true, |p| !area.contains(&p)),
                ]
            })
        })
        .filter(|&is_corner| is_corner)
        .count()
}

pub fn main() {
    let input = fs::read_to_string("input/12.txt").expect("Unable to read input file");
    
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut visited = HashSet::new();
    let mut ans = 0;

    for x in 0..matrix.len() {
        for y in 0..matrix[0].len() {
            if let Some((area, _)) = flood_fill(&matrix, &mut visited, (x, y)) {
                let corners = calculate_corners(&area);
                ans += corners * area.len();
            }
        }
    }
    
    dbg!(ans);
}
