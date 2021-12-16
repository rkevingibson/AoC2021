enum LineResult {
    Corrupted(i32),
    Incomplete(Vec<char>),
}

fn matching_character(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Something messed up"),
    }
}

fn parse(input: &str) -> Vec<LineResult> {
    let score = |c| match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Failed to match"),
    };

    input
        .lines()
        .map(|line| {
            let mut stack = Vec::new();
            let mut chars = line.trim().chars();
            while let Some(c) = chars.next() {
                if c == '(' || c == '[' || c == '{' || c == '<' {
                    stack.push(c);
                } else {
                    let entry = stack.pop().unwrap();

                    if c != matching_character(entry) {
                        return LineResult::Corrupted(score(c));
                    }
                }
            }
            return LineResult::Incomplete(stack);
        })
        .collect()
}

fn part1(input: &str) -> i32 {
    parse(input)
        .iter()
        .map(|result| match result {
            &LineResult::Corrupted(score) => score,
            _ => 0,
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let score = |c| match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Failed to match"),
    };

    let mut line_scores: Vec<u64> = parse(input)
        .iter()
        .map(|result| match result {
            LineResult::Incomplete(stack) => stack
                .iter()
                .rev()
                .map(|&c| score(matching_character(c)))
                .fold(0, |acc, x| 5 * acc + x),
            _ => 0,
        })
        .filter(|&score| score != 0)
        .collect();
    line_scores.sort();
    line_scores[line_scores.len() / 2]
}

fn main() {
    let input = include_str!("../inputs/day10.txt");
    // Parse input here
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[test]
fn test_case() {
    let input = r#"[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]"#;
    assert_eq!(part1(&input), 26397);
    assert_eq!(part2(&input), 288957);
}
