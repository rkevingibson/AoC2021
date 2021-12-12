fn part1(input: &[u32], num_bits: u8) -> u32 {
    let mut gamma = 0;
    // Need to find whether 0 or 1 is more common in each position.
    // Seems like there should be a bit-twiddling trick here, but maybe not.
    // Could do it with masks and sums, easily enough.
    for i in 0..num_bits {
        let mask: u32 = (1 << (i + 1)) - 1;
        let mut count = 0;
        for el in input {
            count += (el & mask) >> i;
        }
        if count > (input.len() / 2).try_into().unwrap() {
            gamma |= 1 << i;
        }
    }
    let epsilon = (!gamma) & ((1 << num_bits) - 1);
    gamma * epsilon
}

fn part2(input: &[u32], num_bits: u8) -> u32 {
    let mut oxygen_rating = 0;
    let mut oxy_candidates = input.to_vec();
    for bit in (0..num_bits).rev() {
        let mask: u32 = (1 << (bit + 1)) - 1;
        let count = oxy_candidates
            .iter()
            .fold(0, |c, el| c + ((el & mask) >> bit));

        // Filter out entries from input
        let filter = ((count * 2) as usize >= oxy_candidates.len()) as u32;
        oxy_candidates = oxy_candidates
            .into_iter()
            .filter(|i| (i & mask) >> bit == filter)
            .collect();

        if oxy_candidates.len() == 1 {
            oxygen_rating = oxy_candidates[0];
            break;
        }
    }

    let mut co_candidates = input.to_vec();
    let mut co_rating = 0;
    for bit in (0..num_bits).rev() {
        let mask: u32 = (1 << (bit + 1)) - 1;
        let count = co_candidates
            .iter()
            .fold(0, |c, el| c + ((el & mask) >> bit));
        // Filter out entries from input
        let filter = ((count * 2) as usize >= co_candidates.len()) as u32;
        co_candidates = co_candidates
            .into_iter()
            .filter(|i| (i & mask) >> bit != filter)
            .collect();

        if co_candidates.len() == 1 {
            co_rating = co_candidates[0];
            break;
        }
    }

    oxygen_rating * co_rating
}

fn main() {
    let input = include_str!("../inputs/day3.txt");
    let input: Vec<u32> = input
        .lines()
        .map(|line| u32::from_str_radix(line.trim(), 2).unwrap())
        .collect();
    println!("Part 1: {:?}", part1(&input, 12));
    println!("Part 2: {:?}", part2(&input, 12));
}

#[test]
fn test() {
    let input = r#"00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010"#;

    let input: Vec<u32> = input
        .lines()
        .map(|line| u32::from_str_radix(line.trim(), 2).unwrap())
        .collect();
    assert_eq!(part1(&input, 5), 198);
    assert_eq!(part2(&input, 5), 230);
}
