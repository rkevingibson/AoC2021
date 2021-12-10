fn part1(input: &[i32]) -> i32 {
    let mut prev = i32::MAX;
    let mut count = 0;
    for &i in input {
        if i > prev {
            count += 1;
        }
        prev = i;
    }
    count
}

fn part2(input: &[i32]) -> i32 {
    let mut count = 0;
    let size = input.len();
    for i in 0..(size - 3) {
        if input[i] < input[i + 3] {
            count += 1;
        }
    }
    count
}

fn main() {
    let input = include_str!("../inputs/day1.txt");
    let values: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    println!("Part 1: {}", part1(&values));
    println!("Part 2: {}", part2(&values))
}

#[test]
fn test_case() {
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(part1(&input), 7);
    assert_eq!(part2(&input), 5);
}
