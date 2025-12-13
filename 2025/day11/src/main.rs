use std::{collections::HashMap, env::args, ffi::FromBytesUntilNulError, path};
use neerajsi::*;

struct PuzzleState<'a> {
    name_to_node: HashMap<&'a str, usize>,
    node_to_name: Vec<&'a str>,
    out_edges: Vec<Vec<usize>>,
}

impl<'a> PuzzleState<'a> {
    fn new() -> Self {
        PuzzleState {
            name_to_node: HashMap::new(),
            node_to_name: Vec::new(),
            out_edges: Vec::new(),
        }
    }

    fn add_node(&mut self, name: &'a str) -> usize {
        if let Some(&node) = self.name_to_node.get(name) {
            return node;
        }

        let node = self.node_to_name.len();
        self.name_to_node.insert(name, node);
        self.node_to_name.push(name);
        self.out_edges.push(Vec::new());
        node
    }

    fn get_node(&self, name: &str) -> Option<usize> {
        self.name_to_node.get(name).copied()
    }
}

const OUT_NODE: usize = 0;

fn main() {
    let input = String::from_utf8(read_stdin_input()).unwrap();

    let mut puzzle_state = PuzzleState::new();
    
    assert_eq!(puzzle_state.add_node("out"), OUT_NODE);

    let puzzle_state = input.lines()
        .map(|line| {
            let (from, to) = line.split_once(":").unwrap();
            (from, to)
        })
        .fold(puzzle_state, |mut state, (from, to)| {
            let from_node = state.add_node(from);
            let to_nodes = to.split_ascii_whitespace()
                .map(|name| state.add_node(name))
                .collect::<Vec<_>>();
            
            assert!(state.out_edges[from_node].is_empty());
            state.out_edges[from_node] = to_nodes;
            state
        });

    let debug = args().any(|arg| arg == "debug");
    let debug1 = debug || args().any(|arg| arg == "debug1");
    let debug2 = debug || args().any(|arg| arg == "debug2");

    detect_cycles(&puzzle_state, "you", debug1);

    let p1 = part1(&puzzle_state, debug1);
    println!("Part 1: {}", p1);

    detect_cycles(&puzzle_state, "svr", debug2);

}

fn detect_cycles(puzzle_state: &PuzzleState, start_node: &str, debug: bool) {
    let node_count = puzzle_state.node_to_name.len();
    let mut visited = vec![false; node_count];
    let mut in_stack = vec![false; node_count];
    let start = puzzle_state.get_node(start_node).unwrap();
    let mut stack = vec![start];
    in_stack[stack[0]] = true;

    while let Some(&node) = stack.last() {
        if visited[node] {
            stack.pop();
            in_stack[node] = false;
            continue;
        }

        if debug {
            println!("Visiting node for cycle detection: {}", puzzle_state.node_to_name[node]);
        }

        visited[node] = true;

        for &neighbor in &puzzle_state.out_edges[node] {
            if !visited[neighbor] {
                stack.push(neighbor);
                in_stack[neighbor] = true;
            } else if in_stack[neighbor] {
                panic!("Cycle detected in the graph at node {}", puzzle_state.node_to_name[neighbor]);
            }
        }
    }
}

fn path_count(puzzle_state: &PuzzleState, start_node: &str, end_node: &str, debug: bool) -> usize {
    let node_count = puzzle_state.node_to_name.len();
    let start = puzzle_state.get_node(start_node).unwrap();
    let mut visited = vec![false; node_count];
    let mut path_to_end_count = vec![0; node_count];

    let end = puzzle_state.get_node(end_node).unwrap();
    path_to_end_count[end] = 1;
    visited[end] = true;

    let mut stack = vec![(start, false)];

    // DFS with post-order processing to add up the
    // count of paths to the end.
    while let Some((node, processed)) = stack.pop() {
        if processed {
            // Post-order processing
            let mut total_paths = 0;
            for &neighbor in &puzzle_state.out_edges[node] {
                total_paths += path_to_end_count[neighbor];
            }
            path_to_end_count[node] = total_paths;

            if debug {
                println!("Node {} has {} paths to end", puzzle_state.node_to_name[node], total_paths);
            }
        } else {
            // Pre-order processing
            if visited[node] {
                continue;
            }
            visited[node] = true;

            stack.push((node, true)); // Mark for post-order processing

            for &neighbor in &puzzle_state.out_edges[node] {
                if !visited[neighbor] {
                    stack.push((neighbor, false));
                }
            }
        }
    }

    path_to_end_count[start]

}

fn part1(puzzle_state: &PuzzleState, debug: bool) -> usize {
    path_count(puzzle_state, "you", "out", debug)
}
