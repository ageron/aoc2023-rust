use hashbrown::{HashMap, HashSet};

fn parse_graph(input: &str) -> HashMap<u32, HashSet<u32>> {
    let mut name_to_index = HashMap::new();
    let mut graph = HashMap::new();
    let mut index = 0;
    input.lines().for_each(|line| {
        let mut parts = line.split(':');
        let from = parts.next().unwrap();
        let from_index = *name_to_index.entry(from).or_insert_with(|| {
            index += 1;
            index - 1
        });
        parts
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .for_each(|to| {
                let to_index = *name_to_index.entry(to).or_insert_with(|| {
                    index += 1;
                    index - 1
                });
                for (index1, index2) in [(to_index, from_index), (from_index, to_index)] {
                    graph
                        .entry(index1)
                        .and_modify(|nodes: &mut HashSet<u32>| {
                            nodes.insert(index2);
                        })
                        .or_insert(HashSet::from([index2]));
                }
            })
    });
    graph
}

fn min_cut_phase(merged_nodes: &[Vec<u32>], edges: &[Vec<u32>]) -> (u32, usize, usize) {
    let num_vertices = merged_nodes.len();
    let mut visited = vec![false; num_vertices];
    let mut num_connections = vec![0; num_vertices];

    let mut min_cuts = u32::MAX;
    let mut last_node = 0;
    let mut previous_node = 0;

    loop {
        let mut most_connected_node = None;
        for node in 0..num_vertices {
            if !merged_nodes[node].is_empty()
                && !visited[node]
                && most_connected_node.map_or(true, |(_, max_connections)| {
                    num_connections[node] > max_connections
                })
            {
                most_connected_node = Some((node, num_connections[node]));
            }
        }
        if let Some((most_connected_node, max_connections)) = most_connected_node {
            previous_node = last_node;
            last_node = most_connected_node;
            min_cuts = max_connections;
            visited[most_connected_node] = true;
            for node in 0..num_vertices {
                if !merged_nodes[node].is_empty() && !visited[node] {
                    num_connections[node] += edges[most_connected_node][node];
                }
            }
        } else {
            return (min_cuts, last_node, previous_node);
        }
    }
}

fn stoer_wagner(graph: &HashMap<u32, HashSet<u32>>) -> (u32, Vec<u32>) {
    let num_vertices = graph.len();
    let mut min_cuts = u32::MAX;
    let mut best_partition1 = vec![];
    let mut merged_nodes: Vec<_> = (0..num_vertices).map(|node| vec![node as u32]).collect();
    let mut edges = vec![vec![0; num_vertices]; num_vertices];

    graph.iter().for_each(|(&from_node, to_nodes)| {
        to_nodes.iter().for_each(|&to_node| {
            edges[from_node as usize][to_node as usize] = 1;
            edges[to_node as usize][from_node as usize] = 1;
        })
    });

    for _ in 1..num_vertices {
        let (num_cuts, last_node, previous_node) = min_cut_phase(&merged_nodes, &edges);
        if num_cuts < min_cuts {
            min_cuts = num_cuts;
            best_partition1 = merged_nodes[last_node].clone();
        }
        let mut last = vec![];
        std::mem::swap(&mut merged_nodes[last_node], &mut last);
        merged_nodes[previous_node].append(&mut last);
        for node in 0..num_vertices {
            if !merged_nodes[node].is_empty() {
                edges[node][previous_node] += edges[node][last_node];
                edges[previous_node][node] = edges[node][previous_node];
            }
        }
    }
    (min_cuts, best_partition1)
}

pub fn run(input: &str) {
    let graph = parse_graph(input);
    let (min_cuts, partition1) = stoer_wagner(&graph);
    assert!(min_cuts == 3);
    let partition1_size = partition1.len();
    let partition2_size = graph.len() - partition1.len();
    println!("{}", partition1_size * partition2_size);
}
