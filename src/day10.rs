use std::time::Instant;
enum Op {
    Noop,
    Addx(i64),
}

#[derive(Default, Copy, Clone)]
struct CpuState {
    register: i64,
    cycle: i64,
    signal_strength: i64,
}

fn part1(input: &str) -> i64 {
    // Care about cycles 20, 60, 100...etc.
    // cycle / 20 = 1, 3, 5.. etc.
    // cycle / 20 % 2 = 1 && cycle % 20 == 0

    let state = input
        .lines()
        .map(|l| {
            if l.as_bytes()[0] == b'n' {
                Op::Noop
            } else {
                let addend = l.split_at(5).1.parse().expect("Bad instruction parse");
                Op::Addx(addend)
            }
        })
        .fold(
            CpuState {
                register: 1,
                ..Default::default()
            },
            |mut cpu, op| {
                match op {
                    Op::Noop => {
                        cpu.cycle += 1;
                        if cpu.cycle % 20 == 0 && (cpu.cycle / 20) % 2 == 1 {
                            cpu.signal_strength += cpu.cycle * cpu.register;
                        }
                    }
                    Op::Addx(addend) => {
                        cpu.cycle += 1;
                        if cpu.cycle % 20 == 0 && (cpu.cycle / 20) % 2 == 1 {
                            cpu.signal_strength += cpu.cycle * cpu.register;
                        }
                        cpu.cycle += 1;
                        if cpu.cycle % 20 == 0 && (cpu.cycle / 20) % 2 == 1 {
                            cpu.signal_strength += cpu.cycle * cpu.register;
                        }
                        cpu.register += addend;
                    }
                };

                cpu
            },
        );

    state.signal_strength
}

fn part2(input: &str) -> [u64; 6] {
    let mut screen = [0u64; 6];

    input
        .lines()
        .map(|l| {
            if l.as_bytes()[0] == b'n' {
                Op::Noop
            } else {
                let addend = l.split_at(5).1.parse().expect("Bad instruction parse");
                Op::Addx(addend)
            }
        })
        .fold(
            CpuState {
                register: 1,
                ..Default::default()
            },
            |mut cpu, op| {
                match op {
                    Op::Noop => {
                        let screen_line = (cpu.cycle) / 40;
                        let screen_pos = (cpu.cycle) % 40;
                        // if screen_pos in interval [reg -1, reg + 1], then we're good.
                        if screen_pos >= cpu.register - 1 && screen_pos <= cpu.register + 1 {
                            screen[screen_line as usize] |= 1 << screen_pos;
                        }
                        cpu.cycle += 1;
                    }
                    Op::Addx(addend) => {
                        // TODO: Could fold these into one check...
                        let screen_line = (cpu.cycle) / 40;
                        let screen_pos = (cpu.cycle) % 40;
                        // if screen_pos in interval [reg -1, reg + 1], then we're good.
                        if screen_pos >= cpu.register - 1 && screen_pos <= cpu.register + 1 {
                            screen[screen_line as usize] |= 1 << screen_pos;
                        }
                        cpu.cycle += 1;
                        let screen_line = (cpu.cycle) / 40;
                        let screen_pos = (cpu.cycle) % 40;
                        // if screen_pos in interval [reg -1, reg + 1], then we're good.
                        if screen_pos >= cpu.register - 1 && screen_pos <= cpu.register + 1 {
                            screen[screen_line as usize] |= 1 << screen_pos;
                        }
                        cpu.cycle += 1;
                        cpu.register += addend;
                    }
                };
                cpu
            },
        );

    screen
}

fn main() {
    let input = include_str!("../inputs/day10.txt");
    let now = Instant::now();
    let p1 = part1(input);
    let p2 = part2(input);
    let dur = now.elapsed();

    println!("Part 1: {}", p1);
    println!("Part 2:");
    for line in p2 {
        for p in 0..40 {
            if line & (1 << p) != 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    println!("Duration: {:?}", dur);
}
