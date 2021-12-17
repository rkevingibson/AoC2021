fn parse_input(input: &str) -> [[u32; 10]; 10] {
    let mut grid = [[0u32; 10]; 10];
    input.lines().enumerate().for_each(|(row, line)| {
        line.trim().chars().enumerate().for_each(|(col, c)| {
            grid[row][col] = c.to_digit(10).unwrap();
        });
    });

    grid
}

fn step_simulation(grid: &mut [[u32; 10]; 10]) {
    // 1. Increase the energy level of each grid cell by 1.
    let mut to_flash = Vec::new();
    let mut has_flashed = [[false; 10]; 10];
    for y in 0..10 {
        for x in 0..10 {
            grid[y][x] += 1;
            if grid[y][x] > 9 {
                to_flash.push((x, y));
                has_flashed[y][x] = true;
            }
        }
    }
    // 2. Any octopus with energy level greater than 9 flashes, increasing energy level of adjacent octopuses.
    while let Some((x, y)) = to_flash.pop() {
        assert!(has_flashed[y][x]);

        let mut update = |x: usize, y: usize| {
            grid[y][x] += 1;
            if grid[y][x] > 9 && !has_flashed[y][x] {
                to_flash.push((x, y));
                has_flashed[y][x] = true;
            }
        };

        if y > 0 {
            update(x, y - 1);
            if x > 0 {
                update(x - 1, y - 1);
            }
            if x + 1 < 10 {
                update(x + 1, y - 1);
            }
        }

        if y + 1 < 10 {
            update(x, y + 1);
            if x > 0 {
                update(x - 1, y + 1);
            }
            if x + 1 < 10 {
                update(x + 1, y + 1);
            }
        }

        if x > 0 {
            update(x - 1, y);
        }
        if x + 1 < 10 {
            update(x + 1, y);
        }
    }

    // 3: Any octopus that flashed is reset
    for y in 0..10 {
        for x in 0..10 {
            if grid[y][x] > 9 {
                grid[y][x] = 0;
            }
        }
    }
}

fn part1(input: &[[u32; 10]; 10]) -> u32 {
    let mut grid = input.clone();
    let mut flash_count = 0u32;
    for _ in 0..100 {
        step_simulation(&mut grid);
        for y in 0..10 {
            for x in 0..10 {
                if grid[y][x] == 0 {
                    flash_count += 1;
                }
            }
        }
    }
    flash_count
}

fn part2(input: &[[u32; 10]; 10]) -> i32 {
    let mut grid = input.clone();
    let mut step_count = 0;
    loop {
        step_count += 1;
        step_simulation(&mut grid);
        if grid.iter().all(|row| row.iter().all(|&x| x == 0)) {
            return step_count;
        }
    }
}

fn main() {
    let input = include_str!("../inputs/day11.txt");
    let input = parse_input(input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[test]
fn test_case() {
    let input = r#"5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526"#;
    let input = parse_input(input);
    println!("{:?}", input);

    assert_eq!(part1(&input), 1656);
    assert_eq!(part2(&input), 195);
}
