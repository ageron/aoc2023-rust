use crate::utils::lcm;
use std::collections::HashMap;

fn get_number_of_steps(
    start: &str,
    end: &str,
    directions: &[u8],
    nodes: &HashMap<&str, [&str; 2]>,
) -> usize {
    let mut current_node: &str = start;
    for n_steps in 0.. {
        if current_node == end || (end.is_empty() && current_node.ends_with('Z')) {
            return n_steps;
        }
        let direction: usize = if directions[n_steps % directions.len()] == b'L' {
            0
        } else {
            1
        };
        current_node = nodes[current_node][direction];
    }
    unreachable!()
}

fn get_number_of_parallel_steps(directions: &[u8], nodes: &HashMap<&str, [&str; 2]>) -> u64 {
    // This implementation assumes that every parallel path hits a Z node at a constant interval
    // (this "Z interval" may be different for every parallel path).
    let start_nodes: Vec<&str> = nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .copied()
        .collect();
    let z_intervals = start_nodes
        .iter()
        .map(|node| get_number_of_steps(node, "", directions, nodes) as u64);
    z_intervals.reduce(lcm).unwrap()
}

pub fn run(input: &str) {
    let mut parts = input.split("\n\n");
    let directions = parts.next().unwrap().as_bytes();
    let nodes: HashMap<_, _> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| (&line[0..3], [&line[7..10], &line[12..15]]))
        .collect();
    let n_steps = get_number_of_steps("AAA", "ZZZ", directions, &nodes);
    println!("{}", n_steps);
    let n_steps = get_number_of_parallel_steps(directions, &nodes);
    println!("{}", n_steps)
}
