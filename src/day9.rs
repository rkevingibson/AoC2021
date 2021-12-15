fn parse_input(s: &str) -> Vec<Vec<u8>> {
    s.lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn part1(input: &[Vec<u8>]) -> i32 {
    let mut risk_level: i32 = 0;
    let rows = input.len();
    for r in 0..rows {
        let cols = input[r].len();
        for c in 0..cols {
            // Can safely get, then unwrap with a default hi value to make the test pass
            let up = if r > 0 { input[r - 1][c] } else { 10 };
            let down = input.get(r + 1).map_or(10,|row| row[c]);
            let left = if c > 0 { input[r][c - 1] } else { 10 };
            let &right = input[r].get(c + 1).unwrap_or(&10);
            let x = input[r][c];
            if x < up && x < down && x < left && x < right {
                risk_level += x as i32 + 1;
            }
        }
    }
    risk_level
}

fn part2(input: &[Vec<u8>]) -> i32 {
    0
}

fn main() {
    let input = include_str!("../inputs/day9.txt");
    let input = parse_input(input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[test]
fn test_case() {
    let input = r#"2199943210
    3987894921
    9856789892
    8767896789
    9899965678"#;
    let input = parse_input(input);
    println!("{:?}", input);
    assert_eq!(part1(&input), 15);
    //assert_eq!(part2(&input), 5);
}
