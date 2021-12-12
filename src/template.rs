fn part1(input: &[i32]) -> i32 {

}

fn part2(input: &[i32]) -> i32 {

}

fn main() {
    let input = include_str!("../inputs/day1.txt");
    // Parse input here
    println!("Part 1: {}", part1(&values));
    println!("Part 2: {}", part2(&values))
}

#[test]
fn test_case() {
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(part1(&input), 7);
    assert_eq!(part2(&input), 5);
}
