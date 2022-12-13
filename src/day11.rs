use std::time::Instant;

#[derive(Copy, Clone)]
enum Op {
    Add(u64),
    Mul(u64),
    Sqr,
}
#[derive(Clone)]
struct Monkey {
    op: Op,
    divisor: u64,
    dst_true: usize,
    dst_false: usize,
    num_inspections: usize,
    items: Vec<u64>,
}

fn part1(input: &mut [Monkey]) -> usize {
    // Do a round of the simulation
    let num_monkeys = input.len();
    for _ in 0..20 {
        for monkey in 0..num_monkeys {
            let src = input[monkey].clone(); // Annoying copy to appease the borrow checker.
            for &item in src.items.iter() {
                let new_worry_level: u64 = match src.op {
                    Op::Add(addend) => item + addend,
                    Op::Mul(scale) => item * scale,
                    Op::Sqr => item * item,
                } / 3;

                let destination = if new_worry_level % src.divisor == 0 {
                    src.dst_true
                } else {
                    src.dst_false
                };

                input[destination].items.push(new_worry_level);
            }
            input[monkey].num_inspections += input[monkey].items.len();
            input[monkey].items.clear();
        }
    }

    let mut inspections: Vec<usize> = input.iter().map(|m| m.num_inspections).collect();
    inspections.sort();
    inspections[num_monkeys - 1] * inspections[num_monkeys - 2]
}

fn part2(input: &mut [Monkey]) -> usize {
    // We need to reduce the numbers somehow, while keeping the results modulo the divisor the same.
    // What modular arithmetic properties are useful here?
    // if a == b mod n, then:
    // a + k == b + k mod n for all k -> This implies reducing by the common multiple won't change the results of adds.
    // k*a == k*b mod n for any integer k -> this implies multiplication by a scalar is fine too.
    // let n be the common multiple?
    //
    // What property do we want to hold? Suppose a is our number before reduction and b is after
    // For every divisor n, we need
    // a + k == b + k mod n
    // a*k == b*k mod n
    // a*a == b*b mod
    // These should all hold if b = a mod n. But we need it to be mod n for all n - which should just be the product, right?

    let common_multiple: u64 = input.iter().map(|m| m.divisor).product();
    let num_monkeys = input.len();
    for _ in 0..10000 {
        for monkey in 0..num_monkeys {
            let src = input[monkey].clone(); // Annoying copy to appease the borrow checker.
            for &item in src.items.iter() {
                let new_worry_level: u64 = match src.op {
                    Op::Add(addend) => item + addend,
                    Op::Mul(scale) => item * scale,
                    Op::Sqr => item * item,
                } % common_multiple;

                let destination = if new_worry_level % src.divisor == 0 {
                    src.dst_true
                } else {
                    src.dst_false
                };

                input[destination].items.push(new_worry_level);
            }
            input[monkey].num_inspections += input[monkey].items.len();
            input[monkey].items.clear();
        }
    }

    let mut inspections: Vec<usize> = input.iter().map(|m| m.num_inspections).collect();
    inspections.sort();
    inspections[num_monkeys - 1] * inspections[num_monkeys - 2]
}

fn main() {
    // let mut input: [Monkey; 4] = [
    //     Monkey {
    //         op: Op::Mul(19),
    //         divisor: 23,
    //         dst_true: 2,
    //         dst_false: 3,
    //         num_inspections: 0,
    //         items: vec![79, 98],
    //     },
    //     Monkey {
    //         op: Op::Add(6),
    //         divisor: 19,
    //         dst_true: 2,
    //         dst_false: 0,
    //         num_inspections: 0,
    //         items: vec![54, 65, 75, 74],
    //     },
    //     Monkey {
    //         op: Op::Sqr,
    //         divisor: 13,
    //         dst_true: 1,
    //         dst_false: 3,
    //         num_inspections: 0,
    //         items: vec![79, 60, 97],
    //     },
    //     Monkey {
    //         op: Op::Add(3),
    //         divisor: 17,
    //         dst_true: 0,
    //         dst_false: 1,
    //         num_inspections: 0,
    //         items: vec![74],
    //     },
    // ];

    let mut input: [Monkey; 8] = [
        Monkey {
            //0
            op: Op::Mul(5),
            divisor: 3,
            dst_true: 7,
            dst_false: 4,
            num_inspections: 0,
            items: vec![66, 71, 94],
        },
        Monkey {
            //1
            op: Op::Add(6),
            divisor: 17,
            dst_true: 3,
            dst_false: 0,
            num_inspections: 0,
            items: vec![70],
        },
        Monkey {
            // 2
            op: Op::Add(5),
            divisor: 2,
            dst_true: 3,
            dst_false: 1,
            num_inspections: 0,
            items: vec![62, 68, 56, 65, 94, 78],
        },
        Monkey {
            //3
            op: Op::Add(2),
            divisor: 19,
            dst_true: 7,
            dst_false: 0,
            num_inspections: 0,
            items: vec![89, 94, 94, 67],
        },
        Monkey {
            // 4
            op: Op::Mul(7),
            divisor: 11,
            dst_true: 5,
            dst_false: 6,
            num_inspections: 0,
            items: vec![71, 61, 73, 65, 98, 98, 63],
        },
        Monkey {
            // 5
            op: Op::Add(7),
            divisor: 5,
            dst_true: 2,
            dst_false: 1,
            num_inspections: 0,
            items: vec![55, 62, 68, 61, 60],
        },
        Monkey {
            // 6
            op: Op::Add(1),
            divisor: 13,
            dst_true: 5,
            dst_false: 2,
            num_inspections: 0,
            items: vec![93, 91, 69, 64, 72, 89, 50, 71],
        },
        Monkey {
            // 7
            op: Op::Sqr,
            divisor: 7,
            dst_true: 4,
            dst_false: 6,
            num_inspections: 0,
            items: vec![76, 50],
        },
    ];

    let mut pt2input = input.clone();

    let now = Instant::now();
    let p1 = part1(&mut input);
    let p2 = part2(&mut pt2input);
    let dur = now.elapsed();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Duration: {:?}", dur);
}
