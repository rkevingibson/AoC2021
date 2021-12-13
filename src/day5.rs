use std::collections::HashMap;

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

fn parse_input(s: &str) -> Vec<Line> {
    s.lines()
        .map(|line| {
            let mut parts = line.split("->");
            let start: Vec<i32> = parts
                .next()
                .unwrap()
                .split(",")
                .map(|piece| piece.trim().parse::<i32>().unwrap())
                .collect();
            let end: Vec<i32> = parts
                .next()
                .unwrap()
                .split(",")
                .map(|piece| piece.trim().parse::<i32>().unwrap())
                .collect();
            Line {
                start: (start[0], start[1]),
                end: (end[0], end[1]),
            }
        })
        .collect()
}

fn part1(input: &[Line]) -> i32 {
    let mut counts = HashMap::<(i32, i32), i32>::new();

    for line in input {
        if line.start.0 == line.end.0 {
            // Vertical line
            let min_y = line.start.1.min(line.end.1);
            let max_y = line.start.1.max(line.end.1);
            for y in min_y..=max_y {
                *counts.entry((line.start.0, y)).or_insert(0) += 1;
            }
        }

        if line.start.1 == line.end.1 {
            // Horizontal line
            let min_x = line.start.0.min(line.end.0);
            let max_x = line.start.0.max(line.end.0);
            for x in min_x..=max_x {
                *counts.entry((x, line.start.1)).or_insert(0) += 1;
            }
        }
    }

    counts.values().fold(0, |acc, &el| {
        if el > 1 {
            return acc + 1;
        } else {
            return acc;
        }
    })
}

fn part2(input: &[Line]) -> i32 {
    let mut counts = HashMap::<(i32, i32), i32>::new();

    for line in input {
        if line.start.0 == line.end.0 {
            // Vertical line
            let min_y = line.start.1.min(line.end.1);
            let max_y = line.start.1.max(line.end.1);
            for y in min_y..=max_y {
                *counts.entry((line.start.0, y)).or_insert(0) += 1;
            }
        }

        if line.start.1 == line.end.1 {
            // Horizontal line
            let min_x = line.start.0.min(line.end.0);
            let max_x = line.start.0.max(line.end.0);
            for x in min_x..=max_x {
                *counts.entry((x, line.start.1)).or_insert(0) += 1;
            }
        }
    }

    counts.values().fold(0, |acc, &el| {
        if el > 1 {
            return acc + 1;
        } else {
            return acc;
        }
    })
}

fn main() {
    let input = include_str!("../inputs/day5.txt");
    let input = parse_input(input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[test]
fn test_case() {
    let input = r#"0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2"#;
    let input = parse_input(input);
    // println!("Input: {:?}", input);
    assert_eq!(part1(&input), 5);
    //assert_eq!(part2(&input), 5);
}
