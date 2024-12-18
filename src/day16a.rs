use std::{collections::{BinaryHeap, HashSet}, fs};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct State {
    x: usize,
    y: usize,
    direction: usize, // 0 = up, 1 = right, 2 = down, 3 = left
    cost: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reverse ordering to make it a min heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star(maze: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> i32 {
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();

    heap.push(State {
        x: start.0,
        y: start.1,
        direction: 1, // facing east initially
        cost: 0,
    });

    while let Some(current) = heap.pop() {
        if (current.x, current.y) == end {
            return current.cost;
        }

        if visited.contains(&(current.x, current.y, current.direction)) {
            continue;
        }

        visited.insert((current.x, current.y, current.direction));

        let next_x = current.x as i32 + directions[current.direction].0;
        let next_y = current.y as i32 + directions[current.direction].1;

        if next_x >= 0
            && next_x < maze[0].len() as i32
            && next_y >= 0
            && next_y < maze.len() as i32
            && maze[next_y as usize][next_x as usize] != '#' {
            heap.push(State {
                x: next_x as usize,
                y: next_y as usize,
                direction: current.direction,
                cost: current.cost + 1,
            });
        }

        for turn in [-1, 1] {
            let new_direction = (current.direction as i32 + turn + 4) as usize % 4;
            heap.push(State {
                x: current.x,
                y: current.y,
                direction: new_direction,
                cost: current.cost + 1000,
            });
        }
    }

    -1
}

pub fn main() {
    let input = fs::read_to_string("input/16.txt").expect("Unable to read input file");

    let maze = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let start = maze.iter().enumerate().find_map(|(y, row)| {
        row.iter().enumerate().find_map(|(x, &cell)| {
            if cell == 'S' {
                Some((x, y))
            } else {
                None
            }
        })
    }).unwrap();

    let end = maze.iter().enumerate().find_map(|(y, row)| {
        row.iter().enumerate().find_map(|(x, &cell)| {
            if cell == 'E' {
                Some((x, y))
            } else {
                None
            }
        })
    }).unwrap();

    let result = a_star(&maze, start, end);

    println!("Result: {}", result);
}
