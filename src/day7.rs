// We have a series of commands, that are fairly easy to parse:
// If the line starts with a $, it's a command, otherwise it's a listing.
// Only commands are cd and ls, so nothing tricky there.
// Directory names are all lowercase it seems like, other than the root.
// All filenames and stuff could be stored as string slices into the input though, so shouldn't be any overhead.

use std::collections::HashMap;
use std::time::Instant;

struct Directory<'a> {
    name: &'a str,
    files: HashMap<&'a str, usize>,
    children: HashMap<&'a str, usize>,
    parent: Option<usize>,
}

fn parse_input(input: &'static str) -> Vec<Directory> {
    let mut lines = input.lines().peekable();

    let mut directories: Vec<Directory> = vec![Directory {
        name: "/",
        files: HashMap::new(),
        children: HashMap::new(),
        parent: None,
    }];

    let mut current_dir: usize = 0;

    while let Some(line) = lines.next() {
        debug_assert!(line.starts_with('$'));
        match line.as_bytes()[2] {
            b'c' => {
                // cd command, set current directory
                match line.split_at(5).1 {
                    "/" => {
                        current_dir = 0;
                    }
                    ".." => {
                        current_dir = directories[current_dir].parent.unwrap();
                    }
                    name @ _ => {
                        if let Some(child) = directories[current_dir].children.get(name) {
                            current_dir = *child;
                        } else {
                            let next_index = directories.len();
                            directories[current_dir].children.insert(name, next_index);
                            directories.push(Directory {
                                name,
                                files: HashMap::new(),
                                children: HashMap::new(),
                                parent: Some(current_dir),
                            });
                            current_dir = next_index;
                        }
                    }
                }
            }
            b'l' => {
                // ls command - parse subsequent lines until we see a new command.
                while let Some(&l) = lines.peek() {
                    if l.starts_with('$') {
                        break;
                    } else if l.starts_with("dir") { // We don't need to do anything with this directory, we'll create it when we enter it.
                    } else {
                        let (size, filename) = l.split_once(' ').unwrap();
                        directories[current_dir]
                            .files
                            .insert(filename, size.parse().expect("Bad parse"));
                    }
                    lines.next();
                }
            }
            _ => {
                panic!("Invalid command");
            }
        }
    }
    directories
}

fn print_directories(dirs: &[Directory]) {
    for d in dirs {
        println!("{}:", d.name);
        println!("\tFiles: {:?}", d.files);
        println!("\tChildren: {:?}", d.children);
    }
}

fn find_size_helper(input: &[Directory], dir: usize) -> (usize, usize) {
    let mut directory_size = input[dir].files.values().sum();
    let mut new_sum = 0;
    for child in input[dir].children.values() {
        let (subtree_size, subtree_score) = find_size_helper(input, *child);
        new_sum += subtree_score;
        directory_size += subtree_size;
    }

    if directory_size < 100000 {
        new_sum += directory_size;
    }
    (directory_size, new_sum)
}

fn part1(input: &[Directory]) -> usize {
    find_size_helper(input, 0).1
}

fn find_smallest_option(input: &[Directory], dir: usize, needed_amount: usize) -> (usize, usize) {
    let mut directory_size = input[dir].files.values().sum();
    let mut best_candidate = usize::MAX;
    for child in input[dir].children.values() {
        let (subtree_size, best_so_far) = find_smallest_option(input, *child, needed_amount);
        directory_size += subtree_size;
        best_candidate = best_candidate.min(best_so_far);
    }

    if directory_size >= needed_amount {
        best_candidate = best_candidate.min(directory_size);
    }
    (directory_size, best_candidate)
}

fn part2(input: &[Directory]) -> usize {
    let free_space = 70_000_000 - find_size_helper(input, 0).0;
    let space_needed = 30_000_000 - free_space;

    find_smallest_option(input, 0, space_needed).1
}

fn main() {
    let input = include_str!("../inputs/day7.txt");
    // Parse input here

    let now = Instant::now();
    let dirs = parse_input(input);
    //print_directories(&dirs);

    let p1 = part1(dirs.as_slice());
    let p2 = part2(dirs.as_slice());
    let dur = now.elapsed();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    println!("Duration: {:?}", dur);
}

#[test]
fn test_case() {}
