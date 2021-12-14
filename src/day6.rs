fn part1(input: &[i32], days: i32) -> usize {
    let mut state = input.to_vec();
    for _ in 0..days {
        let fish_count = state.len();
        for i in 0..fish_count {
            if state[i] == 0 {
                state[i] = 6;
                state.push(8);
            } else {
                state[i] -= 1;
            }
        }
    }

    state.len()
}

fn part2(input: &[i32], days: i32) -> usize {
    // Part 1 works but is too slow for 256 days - can't brute force it, memory grows too much.
    // Need to be cleverer about how quickly it grows.
    // Could represent it "sparsely" - only have values from 0 to 8, so an array of 9 entries with a count of how many fish are in each stage.
    // Updating is then a bit of logic but nothing too hard.
    let mut state: [usize; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for &ele in input {
        state[ele as usize] += 1;
    }

    for _ in 0..days {
        state.rotate_left(1);
        state[6] += state[8];
    }

    state.iter().sum()
}

fn main() {
    let input = include_str!("../inputs/day6.txt");
    let input: Vec<i32> = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    // Parse input here
    println!("Part 1: {}", part1(&input, 80));
    println!("Part 2: {}", part2(&input, 256));
}

#[test]
fn test_case() {
    let input = vec![3, 4, 3, 1, 2];
    assert_eq!(part1(&input, 18), 26);
    assert_eq!(part1(&input, 80), 5934);
    assert_eq!(part2(&input, 256), 26984457539);
}
