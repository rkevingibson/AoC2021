#[derive(Clone, Copy)]
enum Fold {
    Vertical(i32),
    Horizontal(i32),
}

fn parse_input(s: &str) -> (Vec<(i32, i32)>, Vec<Fold>) {
    let mut lines = s.lines();
    let mut points = Vec::new();
    let mut folds = Vec::new();
    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("fold along") {
            let split_point = line.find('=').unwrap();
            let (prefix, num) = line.split_at(split_point + 1);
            let num: i32 = num.parse().unwrap();
            if prefix.contains("x=") {
                folds.push(Fold::Horizontal(num));
            } else {
                folds.push(Fold::Vertical(num));
            }
        } else {
            let split_point = line.find(',').unwrap();
            let (a, b) = line.split_at(split_point);
            points.push((a.parse().unwrap(), b[1..].parse().unwrap()));
        }
    }

    (points, folds)
}

fn do_fold(input: &[(i32, i32)], fold: Fold) -> Vec<(i32, i32)> {
    match fold {
        Fold::Horizontal(f) => input
            .iter()
            .map(|pt: &(i32, i32)| -> (i32, i32) {
                if pt.0 < f {
                    *pt
                } else {
                    (2 * f - pt.0, pt.1)
                }
            })
            .collect(),
        Fold::Vertical(f) => input
            .iter()
            .map(|pt: &(i32, i32)| -> (i32, i32) {
                if pt.1 < f {
                    *pt
                } else {
                    (pt.0, 2 * f - pt.1)
                }
            })
            .collect(),
    }
}

fn part1(input: &[(i32, i32)], fold: Fold) -> i32 {
    let mut result = do_fold(input, fold);
    result.sort();
    result.dedup();
    result.len() as i32
}

fn part2(input: &[(i32, i32)], folds: &[Fold]) {
    let mut result= input.to_vec();
    for f in folds.iter() {
        result = do_fold(&result, *f);
    }
    result.sort();
    result.dedup();

    // Now need to "plot" this result.
    // We can create a "bitmap" of 0s and 1s.
    let &size = result.last().unwrap();
    let mut bitmap = Vec::<bool>::new();
    bitmap.resize(((size.0 + 1)*(size.1 + 1)) as usize, false);
    for pt in result {
        bitmap[(pt.0 + (size.0 + 1)*pt.1) as usize] = true;
    }

    for i in 0..((size.0 + 1)*(size.1 + 1)) {
        if i % (size.0 + 1) == 0 {
            print!("\n");
        }

        if bitmap[i as usize] {
            print!("#");
        } else {
            print!(".");
        }
    }
}

fn main() {
    let input = include_str!("../inputs/day13.txt");
    let (points, folds) = parse_input(input);
    println!("Part 1: {}", part1(&points, folds[0]));
    part2(&points, &folds);
}

#[test]
fn test_case() {
    let input = r#"6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0
    
    fold along y=7
    fold along x=5"#;
    let (points, folds) = parse_input(input);

    assert_eq!(part1(&points, folds[0]), 17);
}
