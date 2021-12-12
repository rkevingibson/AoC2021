struct Board {
    nums: [i32; 25],
    set: [bool; 25],
}

impl Board {
    fn set_num(&mut self, n: i32) {
        let pos = self.nums.iter().position(|x| *x == n);
        match pos {
            Some(i) => self.set[i] = true,
            None => (),
        }
    }

    fn is_winner(&self) -> bool {
        //if (set)
        false
    }
}

fn parse_input(s: &str) {
    // First line is the inputs, followed by boards
}

fn part1(input: &[i32]) -> i32 {
    0
}

fn part2(input: &[i32]) -> i32 {
    0
}

fn main() {
    let input = include_str!("../inputs/day4.txt");
    // Parse input here
    println!("Part 1: {}", part1(&values));
    println!("Part 2: {}", part2(&values))
}

#[test]
fn test_case() {
    let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7"#;
    assert_eq!(part1(&input), 7);
    assert_eq!(part2(&input), 5);
}
