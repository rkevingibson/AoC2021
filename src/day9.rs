use std::collections::HashSet;
use std::time::Instant;

fn special_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs().max((a.1 - b.1).abs())
}

fn rope_simulation(input: &str, rope_length: usize) -> usize {
    let mut tail_pos_history: HashSet<(i32, i32)> = HashSet::new();

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    tail_pos_history.insert(tail_pos);

    let mut rope_positions = vec![(0, 0); rope_length];

    for line in input.lines() {
        let (dir, count) = line.split_at(2);
        let dir = dir.as_bytes()[0]; // We know dir is a single ascii character
        let count = count.parse::<i32>().expect("Invalid count");

        let dir: (i32, i32) = match dir {
            b'R' => (1, 0),
            b'U' => (0, 1),
            b'L' => (-1, 0),
            b'D' => (0, -1),
            _ => panic!("Bad direction"),
        };

        for t in 1..=count {
            let pos = (head_pos.0 + t * dir.0, head_pos.1 + t * dir.1);

            if special_distance(pos, tail_pos) <= 1 {
                // No movement needed from the tail
            } else {
                // Need to move tail to keep up.
                // The tail's next position is always pos - dir
                let next_tail_pos = (pos.0 - dir.0, pos.1 - dir.1);
                tail_pos_history.insert(next_tail_pos);
                tail_pos = next_tail_pos;
            }
        }

        head_pos = (head_pos.0 + count * dir.0, head_pos.1 + count * dir.1);
    }

    tail_pos_history.len()
}

fn part1(input: &str) -> usize {
    rope_simulation(input, 1)
}

fn part2(input: &str) -> usize {
    rope_simulation(input, 9)
}

fn main() {
    let input = include_str!("../inputs/day9.txt");
    let now = Instant::now();
    let p1 = part1(input);
    let p2 = part2(input);
    let dur = now.elapsed();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Duration: {:?}", dur);
}
