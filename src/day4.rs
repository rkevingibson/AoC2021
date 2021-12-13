struct Board {
    nums: [i32; 25],
    set: u32,
}

impl Board {
    fn new(nums: &[i32]) -> Board {
        assert!(nums.len() == 25);
        let mut b = Board {
            nums: Default::default(),
            set: 0,
        };
        b.nums.copy_from_slice(nums);
        b
    }

    fn reset(&mut self) {
        self.set = 0;
    }

    fn set_num(&mut self, n: i32) {
        let pos = self.nums.iter().position(|x| *x == n);
        match pos {
            Some(i) => self.set |= 1 << i,
            None => (),
        }
    }

    fn is_winner(&self) -> bool {
        let wins = [
            0b00000_00000_00000_00000_11111u32,
            0b00000_00000_00000_11111_00000u32,
            0b00000_00000_11111_00000_00000u32,
            0b00000_11111_00000_00000_00000u32,
            0b11111_00000_00000_00000_00000u32,
            0b10000_10000_10000_10000_10000u32,
            0b01000_01000_01000_01000_01000u32,
            0b00100_00100_00100_00100_00100u32,
            0b00010_00010_00010_00010_00010u32,
            0b00001_00001_00001_00001_00001u32,
        ];
        wins.iter().any(|w| *w & self.set == *w)
    }

    fn score(&self) -> i32 {
        let mut sum = 0;
        for i in 0..25 {
            if self.set & (1 << i) == 0 {
                sum += self.nums[i];
            }
        }
        sum
    }
}

fn parse_input(s: &str) -> (Vec<i32>, Vec<Board>) {
    // First line is the inputs, followed by boards
    let mut lines = s.lines();
    let calls = lines
        .next()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .unwrap();
    // We get 5 numbers per line, need to collect 25
    let mut boards = Vec::<Board>::new();
    let mut nums = Vec::<i32>::new();
    for line in lines {
        line.split_whitespace()
            .map(|s| s.parse::<i32>())
            .for_each(|el| {
                if let Ok(e) = el {
                    nums.push(e);
                }
            });
        if nums.len() == 25 {
            boards.push(Board::new(&nums));
            nums.clear();
        }
    }

    (calls, boards)
}

fn part1(calls: &[i32], boards: &mut [Board]) -> i32 {
    for &call in calls {
        for i in 0..boards.len() {
            boards[i].set_num(call);
        }
        let winner = boards.iter().find(|b| b.is_winner());
        if let Some(winning_board) = winner {
            println!("Called {}", call);
            println!("Board score {}", winning_board.score());
            return winning_board.score() * call;
        }
    }
    0
}

fn part2(calls: &[i32], mut boards: Vec<Board>) -> i32 {
    for &call in calls {
        for i in 0..boards.len() {
            boards[i].set_num(call);
        }
        boards = boards.into_iter().filter(|b| !b.is_winner()).collect();

        if boards.len() == 1 {
            // Final board, need to find score when we lose:
            break;
        }
    }

    let losing_board = &mut boards[0];
    losing_board.reset();
    for &call in calls {
        losing_board.set_num(call);
        if losing_board.is_winner() {
            return losing_board.score() * call;
        }
    }
    0
}

fn main() {
    let input = include_str!("../inputs/day4.txt");
    let (calls, mut boards) = parse_input(input);
    // Parse input here
    println!("Part 1: {}", part1(&calls, &mut boards));
    for ele in &mut boards {
        ele.reset();
    }
    println!("Part 2: {}", part2(&calls, boards));
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
    let (calls, mut boards) = parse_input(input);

    assert_eq!(part1(&calls, &mut boards), 4512);
    for ele in &mut boards {
        ele.reset();
    }
    assert_eq!(part2(&calls, boards), 1924);
}
