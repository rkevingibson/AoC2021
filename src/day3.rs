use std::time::Instant;

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let (first, second) = s.split_at(s.len() / 2); // Split the lines in half
            let mut set1 = 0u64;
            let mut set2 = 0u64;
            for &c in first.as_bytes() {
                set1 |= 1 << char_to_index(c);
            }
            for &c in second.as_bytes() {
                set2 |= 1 << char_to_index(c);
            }
            let priority = (set1 & set2).trailing_zeros();
            priority
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let mut matches: u64 = !0;
    for (i, l) in input.lines().enumerate() {
        let mut set = 0u64;
        for &c in l.as_bytes().iter() {
            set |= 1 << char_to_index(c);
        }
        matches &= set;

        if i % 3 == 2 {
            sum += matches.trailing_zeros();
            matches = !0;
        }
    }
    sum
}

#[inline]
fn char_to_index(c: u8) -> u8 {
    (c - b'A' + 27) % 58
}

fn main() {
    let input = include_str!("../inputs/day3.txt");
    let now = Instant::now();
    let p1 = part1(input);

    let p2 = part2(input);
    let dur = now.elapsed();
    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", p2);
    println!("Duration: {:?}", dur);
}

#[test]
fn test() {
    let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    assert_eq!(part1(input), 157);
    assert_eq!(part2(input), 70);
}
