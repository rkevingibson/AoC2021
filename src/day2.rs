#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

fn parse_input(input: &str) -> Vec<(Direction, i32)> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let dir = match iter.next() {
                Some("forward") => Some(Direction::Forward),
                Some("down") => Some(Direction::Down),
                Some("up") => Some(Direction::Up),
                _ => None,
            };
            let count = iter.next().map(|s| s.parse::<i32>().unwrap());
            (dir.unwrap(), count.unwrap())
        })
        .collect()
}

fn part1(input: &[(Direction, i32)]) -> i32 {
    let mut state = (0, 0);
    for ele in input {
        match ele.0 {
            Direction::Forward => state.0 += ele.1,
            Direction::Down => state.1 += ele.1,
            Direction::Up => state.1 -= ele.1,
        }
    }
    state.0 * state.1
}

fn part2(input: &[(Direction, i32)]) -> i32 {
    // State = horizontal position, depth, aim
    let mut state = (0, 0, 0);
    for ele in input {
        match ele.0 {
            Direction::Forward => {
                state.0 += ele.1;
                state.1 += ele.1 * state.2
            }
            Direction::Down => state.2 += ele.1,
            Direction::Up => state.2 -= ele.1,
        }
    }
    state.0 * state.1
}

fn main() {
    let input = include_str!("../inputs/day2.txt");
    let input = parse_input(input);
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

#[test]
fn test() {
    let input = r#"forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2"#;
    let input = parse_input(input);
    println!("Test input: {:?}", input);
    assert_eq!(part1(&input), 150);
    assert_eq!(part2(&input), 900);
}
