

fn main() {
    let input = include_str!("../inputs/day2.txt");
    let values: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    println!("Part 1: {}", part1(&values));
    println!("Part 2: {}", part2(&values))
}

#[test]
fn test() {
    let input = r#"forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2"#;
}