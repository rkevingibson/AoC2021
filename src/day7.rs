fn part1(input: &[i32]) -> i32 
{
    let mut sorted_input = input.to_vec();
    sorted_input.sort();

    let median : i32 = sorted_input[sorted_input.len()/2];
    println!("Median {}", median);
    let fuel_usage = input.iter().fold(0, |acc, x| {
        acc + (x-median).abs()
    });

    fuel_usage
}

fn part2(input: &[i32]) -> i32 
{
    let mean = (input.iter().sum::<i32>() as f64 / input.len() as f64).round() as i32;
    println!("Mean {}", mean);
    let mut f = i32::MAX;
    // Concerned about roundoff error here - suspect I should be doing something more stable for computing mean.
    for candidate in (mean-1)..(mean+1) {
        let fuel_usage = input.iter().fold(0, |acc, x| {
            let n = (x - candidate).abs();
            acc + n*(n+1)/2
        });
        if fuel_usage < f {
            println!("Current best: {}", candidate);
        }
        f = f.min(fuel_usage);
    }
    f
}

fn main() {
    let input = include_str!("../inputs/day7.txt");
    let input: Vec<i32> = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    // Parse input here
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[test]
fn test_case() {
    let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    assert_eq!(part1(&input), 37);
    assert_eq!(part2(&input), 168);
}
