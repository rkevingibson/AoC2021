use std::collections::VecDeque;
use std::time::Instant;

#[derive(Default, Clone, Copy, Debug)]
struct Node {
    distance: Option<usize>, // None if it's not been visited.
    height: u8,              // Height is 0-25, with special
}

fn part1(input: &str) -> usize {
    let width = input.find(char::is_whitespace).unwrap();
    let height = input.len() / (width + 2) + 1; // We have 2 whitespace characters as newlines on windows.

    let mut grid: Vec<Node> = input
        .as_bytes()
        .iter()
        .filter_map(|&c| {
            if c.is_ascii_whitespace() {
                None
            } else {
                Some(Node {
                    distance: None,
                    height: c,
                })
            }
        })
        .collect();

    // First, need to find the start index.
    let start = grid
        .iter()
        .enumerate()
        .find(|(_, &node)| node.height == b'S')
        .unwrap()
        .0;

    grid[start].distance = Some(0);
    grid[start].height = b'a';

    let end = grid
        .iter()
        .enumerate()
        .find(|(_, node)| node.height == b'E')
        .unwrap()
        .0;
    grid[end].height = b'z';

    let mut stack = VecDeque::from([start]);

    while let Some(node_index) = stack.pop_front() {
        // Add neighbour indices to the stack if it hasn't been visited.
        let next_dist = *grid[node_index]
            .distance
            .as_ref()
            .expect("Visited a node without a distance")
            + 1;

        let current_height = grid[node_index].height;
        let node_x = node_index % width;
        let node_y = node_index / width;

        if node_index == end {
            return next_dist - 1;
        }

        if node_y > 0 {
            let up = node_index - width;

            let next = &mut grid[up];
            if next.distance.is_none() && next.height <= current_height + 1 {
                next.distance = Some(next_dist);
                stack.push_back(up);
            }
        }
        if node_y + 1 < height {
            let down = node_index + width;

            let next = &mut grid[down];
            if next.distance.is_none() && next.height <= current_height + 1 {
                next.distance = Some(next_dist);
                stack.push_back(down);
            }
        }
        if node_x > 0 {
            let left = node_index - 1;
            let next = &mut grid[left];
            if next.distance.is_none() && next.height <= current_height + 1 {
                next.distance = Some(next_dist);
                stack.push_back(left);
            }
        }

        if node_x + 1 < width {
            let right = node_index + 1;
            let next = &mut grid[right];
            if next.distance.is_none() && next.height <= current_height + 1 {
                next.distance = Some(next_dist);
                stack.push_back(right);
            }
        }
    }
    0
}

fn part2(input: &str) -> usize {
    let width = input.find(char::is_whitespace).unwrap();
    let height = input.len() / (width + 2) + 1; // We have 2 whitespace characters as newlines on windows.

    let mut grid: Vec<Node> = input
        .as_bytes()
        .iter()
        .filter_map(|&c| {
            if c.is_ascii_whitespace() {
                None
            } else {
                Some(Node {
                    distance: None,
                    height: c,
                })
            }
        })
        .collect();

    // First, need to find the start index.
    let start = grid
        .iter()
        .enumerate()
        .find(|(_, &node)| node.height == b'S')
        .unwrap()
        .0;

    grid[start].height = b'a';

    let end = grid
        .iter()
        .enumerate()
        .find(|(_, node)| node.height == b'E')
        .unwrap()
        .0;
    grid[end].height = b'z';
    grid[end].distance = Some(0);

    // Start at the end, find shortest distance to 'a'
    let mut stack = VecDeque::from([end]);

    while let Some(node_index) = stack.pop_front() {
        // Add neighbour indices to the stack if it hasn't been visited.
        let next_dist = *grid[node_index]
            .distance
            .as_ref()
            .expect("Visited a node without a distance")
            + 1;

        let current_height = grid[node_index].height;
        let node_x = node_index % width;
        let node_y = node_index / width;

        if current_height == b'a' {
            return next_dist - 1;
        }

        if node_y > 0 {
            let up = node_index - width;

            let next = &mut grid[up];
            if next.distance.is_none() && next.height >= current_height - 1 {
                next.distance = Some(next_dist);
                stack.push_back(up);
            }
        }
        if node_y + 1 < height {
            let down = node_index + width;

            let next = &mut grid[down];
            if next.distance.is_none() && next.height >= current_height - 1 {
                next.distance = Some(next_dist);
                stack.push_back(down);
            }
        }
        if node_x > 0 {
            let left = node_index - 1;
            let next = &mut grid[left];
            if next.distance.is_none() && next.height >= current_height - 1 {
                next.distance = Some(next_dist);
                stack.push_back(left);
            }
        }

        if node_x + 1 < width {
            let right = node_index + 1;
            let next = &mut grid[right];
            if next.distance.is_none() && next.height >= current_height - 1 {
                next.distance = Some(next_dist);
                stack.push_back(right);
            }
        }
    }
    0
}

fn main() {
    let input = include_str!("../inputs/day12.txt");
    let now = Instant::now();
    let p1 = part1(input);
    let p2 = part2(input);
    let dur = now.elapsed();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Duration: {:?}", dur);
}
