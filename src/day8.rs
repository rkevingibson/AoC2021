use std::collections::HashMap;

fn str_to_digit(s: &str) -> u8 {
    let mut digit: u8 = 0;
    for c in s.chars() {
        match c {
            'a' => digit |= 0b000_0001u8,
            'b' => digit |= 0b000_0010u8,
            'c' => digit |= 0b000_0100u8,
            'd' => digit |= 0b000_1000u8,
            'e' => digit |= 0b001_0000u8,
            'f' => digit |= 0b010_0000u8,
            'g' => digit |= 0b100_0000u8,
            _ => (),
        }
    }

    digit
}

fn parse_input(s: &'static str) -> Vec<(Vec<u8>, Vec<u8>)> {
    s.lines()
        .map(|line| {
            let (inputs, outputs) = line.split_at(line.find('|').unwrap());

            (
                inputs
                    .split_whitespace()
                    .map(|piece| str_to_digit(piece))
                    .collect(),
                outputs[1..]
                    .split_whitespace()
                    .map(|piece| str_to_digit(piece))
                    .collect(),
            )
        })
        .collect()
}

fn part1(input: &[(Vec<u8>, Vec<u8>)]) -> i32 {
    input.iter().fold(0, |sum, entry| {
        sum + entry.1.iter().fold(0, |acc, &s| {
            if s.count_ones() == 2
                || s.count_ones() == 4
                || s.count_ones() == 3
                || s.count_ones() == 7
            {
                acc + 1
            } else {
                acc
            }
        })
    })
}

fn part2(input: &[(Vec<u8>, Vec<u8>)]) -> u32 {
    // For each row, need to first inspect the inputs only, to determine the outputs.
    // How to do this mapping to signal segment, and how to represent it?
    // For each line, trying to find a mapping from unique string to number. (Or from 8 bit digit to number, equivalently)
    // We know if len == 2 -> num == 1
    // If len == 4 -> num == 4
    // if len == 3 -> num == 7
    // If len == 7 -> num == 8
    // Could map all the strings to 8 bit numbers, maybe do it with bit manipulation for the rest?
    // 0 -> length 6 - contains 1 as a substring, doesn't have 4.
    // 1 -> length 2 **
    // 2 -> length 5 - left behind after 3 and 5 are matched.
    // 3 -> length 5 - contains 1 (and 7) as a substring, unique among 5-digit ones
    // 4 -> length 4 **
    // 5 -> length 5 - subtract 1 from 4, gives bd, unique among 5-digit ones.
    // 6 -> length 6 - last one left for 6-digits.
    // 7 -> length 3 **
    // 8 -> length 7 **
    // 9 -> length 6 - contains 4 as a substring
    // So that should be sufficient to write out the logic, how to represent things though?
    // And what I'm trying to do is for each line: need a mapping from input digit to [0,9] range.
    // Should always have enough info in inputs - shows all ten input patterns
    // Substring/contains tests will be pricy without converting to ints first.
    // Len becomes popcount.
    // This feels like it could become simpler with some extra bit manipulation, but maybe not.

    let num_entries = input.len();
    let mut sum = 0;
    for i in 0..num_entries {
        let mut mapping = HashMap::<u8, u8>::new();
        let row = &input[i].0;
        // Can find the trivial cases first:
        let &one = row.iter().find(|&&s| s.count_ones() == 2).unwrap();
        let &seven = row.iter().find(|&&s| s.count_ones() == 3).unwrap();
        let &four = row.iter().find(|&&s| s.count_ones() == 4).unwrap();
        let &eight = row.iter().find(|&&s| s.count_ones() == 7).unwrap();
        let &three = row
            .iter()
            .find(|&&s| s.count_ones() == 5 && (s & seven) == seven)
            .unwrap();
        let bd = one ^ four;
        let &five = row
            .iter()
            .find(|&&s| s.count_ones() == 5 && (s & bd) == bd)
            .unwrap();
        let &two = row
            .iter()
            .find(|&&s| s.count_ones() == 5 && s != three && s != five)
            .unwrap();

        let &nine = row
            .iter()
            .find(|&&s| s.count_ones() == 6 && (s & four) == four)
            .unwrap();

        let &zero = row
            .iter()
            .find(|&&s| s.count_ones() == 6 && (s & one) == one && s != nine)
            .unwrap();
        let &six = row
            .iter()
            .find(|&&s| s.count_ones() == 6 && s != zero && s != nine)
            .unwrap();

        mapping.insert(zero, 0);
        mapping.insert(one, 1);
        mapping.insert(two, 2);
        mapping.insert(three, 3);
        mapping.insert(four, 4);
        mapping.insert(five, 5);
        mapping.insert(six, 6);
        mapping.insert(seven, 7);
        mapping.insert(eight, 8);
        mapping.insert(nine, 9);

        let output = &input[i].1;
        assert_eq!(output.len(), 4);
        let output = mapping[&output[0]] as u32 * 1000
            + mapping[&output[1]] as u32 * 100
            + mapping[&output[2]] as u32 * 10
            + mapping[&output[3]] as u32;
        sum += output;
    }

    sum
}

fn main() {
    let input = include_str!("../inputs/day8.txt");
    let input = parse_input(input);
    // Parse input here
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[test]
fn test_case() {
    let input = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;
    let input = parse_input(input);
    assert_eq!(part1(&input), 26);
    assert_eq!(part2(&input), 61229);
}
