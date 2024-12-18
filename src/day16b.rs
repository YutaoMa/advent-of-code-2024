use std::{collections::{BinaryHeap, HashMap, HashSet}, fs};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct State {
    x: usize,
    y: usize,
    direction: usize, // 0 = up, 1 = right, 2 = down, 3 = left
    cost: i32,
    path: Vec<(usize, usize)>,
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

fn a_star_track(maze: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> HashSet<(usize, usize)> {
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut visited: HashMap<(usize, usize, usize), i32> = HashMap::new();
    let mut heap = BinaryHeap::new();

    let mut best_cost = i32::MAX;
    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();

    heap.push(State {
        x: start.0,
        y: start.1,
        direction: 1, // facing east initially
        cost: 0,
        path: vec![start],
    });

    while let Some(current) = heap.pop() {
        if current.cost > best_cost {
            continue;
        }

        if (current.x, current.y) == end {
            if current.cost < best_cost {
                best_cost = current.cost;
                paths.clear();
            }

            paths.push(current.path.clone());
            continue;
        }

        if let Some(&prev_cost) = visited.get(&(current.x, current.y, current.direction)) {
            if current.cost > prev_cost {
                continue;
            }
        }

        visited.insert((current.x, current.y, current.direction), current.cost);

        let next_x = current.x as i32 + directions[current.direction].0;
        let next_y = current.y as i32 + directions[current.direction].1;

        if next_x >= 0
            && next_x < maze[0].len() as i32
            && next_y >= 0
            && next_y < maze.len() as i32
            && maze[next_y as usize][next_x as usize] != '#' {
            let mut new_path = current.path.clone();
            new_path.push((next_x as usize, next_y as usize));

            heap.push(State {
                x: next_x as usize,
                y: next_y as usize,
                direction: current.direction,
                cost: current.cost + 1,
                path: new_path,
            });
        }

        for turn in [-1, 1] {
            let new_direction = (current.direction as i32 + turn + 4) as usize % 4;
            heap.push(State {
                x: current.x,
                y: current.y,
                direction: new_direction,
                cost: current.cost + 1000,
                path: current.path.clone(),
            });
        }
    }

    let mut union_points: HashSet<(usize, usize)> = HashSet::new();
    for path in paths {
        for point in path {
            union_points.insert(point);
        }
    }

    union_points
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

    let result = a_star_track(&maze, start, end);

    println!("Result: {}", result.len());
}
