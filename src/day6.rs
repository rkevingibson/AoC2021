use std::time::Instant;

fn find_unique_window(input: &str, window_size: usize) -> usize {
    let mut set: usize = input.as_bytes()[0..window_size - 1]
        .iter()
        .fold(0, |acc, &c| acc ^ 1 << (c - b'a'));
    input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .find_map(|(index, win)| {
            set ^= 1 << (win.last().unwrap() - b'a');
            if set.count_ones() == window_size.try_into().unwrap() {
                return Some(index + window_size);
            }
            set ^= 1 << (win.first().unwrap() - b'a');
            return None;
        })
        .unwrap()
}

fn part1(input: &str) -> usize {
    find_unique_window(input, 4)
}

fn part2(input: &str) -> usize {
    find_unique_window(input, 14)
}

fn main() {
    let input = include_str!("../inputs/day6.txt");

    let now = Instant::now();
    let p1 = part1(input);
    let p2 = part2(input);
    let dur = now.elapsed();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Duration: {:?}", dur);
}

#[test]
fn test_case() {
    assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
}
