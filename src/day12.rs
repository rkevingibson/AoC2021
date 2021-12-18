/**
 * Graph traversal: can visit certain nodes multiple times, others only once.
 * Should be a reasonably straightforward DFS/BFS write up, but with some tracking over whether node is visited or not.
 * But for creating a graph, we're now in memory management territory where my weak rust will show.
 * For simplicity, I'll just use Rc for the nodes.
 */
use std::rc::Rc;

enum NodeType {
    Start,
    End,
    Large(&'static str),
    Small(&'static str),
}

struct Node {
    edges: Vec<Rc<Node>>,
    node_type: NodeType,
}

fn parse_input(input: &'static str) -> Rc<Node> {
    let mut nodes = Vec::<Rc<Node>>::new();

    input.lines().map(|line| line);

    Rc::new(Node {
        edges: Vec::new(),
        node_type: NodeType::Start,
    })
}

fn part1(input: Rc<Node>) -> i32 {
    0
}

fn part2(input: Rc<Node>) -> i32 {
    0
}

fn main() {
    let input = include_str!("../inputs/day12.txt");
    let input = parse_input(input);
    println!("Part 1: {}", part1(Rc::clone(&input)));
    println!("Part 2: {}", part2(Rc::clone(&input)));
}

#[test]
fn test_case() {
    let input = r#"start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end"#;
    let input = parse_input(input);
    assert_eq!(part1(Rc::clone(&input)), 10);
}
