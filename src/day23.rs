use hashbrown::HashMap;

use std::collections::VecDeque;

type Graph = HashMap<(i32, i32), Vec<((i32, i32), u32)>>;

fn successors(pos: (i32, i32), grid: &[Vec<u8>], is_slippery: bool) -> Vec<((i32, i32), u32)> {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(dx, dy)| {
            let (nx, ny) = (pos.0 + dx, pos.1 + dy);
            if nx < 0 || nx >= width || ny < 0 || ny >= height {
                return None;
            }
            match grid[ny as usize][nx as usize] {
                b'#' => {
                    return None;
                }
                b'<' => {
                    if is_slippery && nx - pos.0 == 1 {
                        return None;
                    }
                }
                b'>' => {
                    if is_slippery && nx - pos.0 == -1 {
                        return None;
                    }
                }
                b'^' => {
                    if is_slippery && ny - pos.1 == 1 {
                        return None;
                    }
                }
                b'v' => {
                    if is_slippery && ny - pos.1 == -1 {
                        return None;
                    }
                }
                b'.' => {}
                _ => unreachable!(),
            }
            Some(((nx, ny), 1))
        })
        .collect()
}

fn grid_to_graph(grid: &[Vec<u8>], start: (i32, i32), end: (i32, i32), is_slippery: bool) -> Graph {
    let mut graph = HashMap::new();
    let mut to_visit = VecDeque::from([start]);
    while let Some(pos) = to_visit.pop_front() {
        let next = if pos == end {
            vec![]
        } else {
            successors(pos, grid, is_slippery)
        };
        for &(pos, _) in &next {
            if !graph.contains_key(&pos) && pos != end {
                to_visit.push_back(pos);
            }
        }
        graph.insert(pos, next);
    }
    graph
}

fn keep_only_longest_to_same_destination(next: &[((i32, i32), u32)]) -> Vec<((i32, i32), u32)> {
    let mut longest: HashMap<(i32, i32), u32> = HashMap::new();
    next.iter().for_each(|&(pos, distance)| {
        longest
            .entry(pos)
            .and_modify(|d| {
                if *d < distance {
                    *d = distance;
                }
            })
            .or_insert(distance);
    });
    longest.into_iter().collect()
}

fn graph_simplification_step(graph: &Graph, start: (i32, i32), end: (i32, i32)) -> Graph {
    let mut simpler_graph = HashMap::new();
    let mut to_visit = VecDeque::from([start]);
    while let Some(crossroad_pos) = to_visit.pop_front() {
        if simpler_graph.contains_key(&crossroad_pos) {
            continue;
        }
        let mut next_crossroads = vec![];
        'branches: for &(branch_start, distance) in graph.get(&crossroad_pos).unwrap_or(&vec![]) {
            let mut steps = distance;
            let mut prev_pos = crossroad_pos;
            let mut pos = branch_start;
            loop {
                if pos == crossroad_pos || pos == start {
                    continue 'branches;
                }
                if pos == end {
                    break;
                }
                let mut next = graph.get(&pos).unwrap_or(&vec![]).clone();
                next.retain(|(next_pos, _)| *next_pos != prev_pos);
                match next.len() {
                    0 => {
                        continue 'branches;
                    }
                    1 => {
                        prev_pos = pos;
                        pos = next[0].0;
                        steps += next[0].1;
                    }
                    _ => {
                        break;
                    }
                };
            }
            next_crossroads.push((pos, steps));
        }
        simpler_graph.insert(
            crossroad_pos,
            keep_only_longest_to_same_destination(&next_crossroads),
        );
        next_crossroads
            .into_iter()
            .for_each(|(pos, _)| to_visit.push_back(pos))
    }

    simpler_graph
}

fn simplify_graph(graph: &Graph, start: (i32, i32), end: (i32, i32)) -> Graph {
    let mut graph_size = graph.len();
    let mut simpler_graph = graph_simplification_step(graph, start, end);
    while simpler_graph.len() < graph_size {
        graph_size = simpler_graph.len();
        simpler_graph = graph_simplification_step(graph, start, end);
    }
    simpler_graph
}

fn find_longest_path(graph: &Graph, start: (i32, i32), end: (i32, i32)) -> Option<u32> {
    if start == end {
        return Some(0);
    }
    let mut graph = simplify_graph(graph, start, end);

    let next = graph.remove(&start);
    if let Some(next) = next {
        next.iter()
            .filter_map(|&(next_pos, distance)| {
                find_longest_path(&graph, next_pos, end)
                    .map(|longest_remaining| longest_remaining + distance)
            })
            .max()
    } else {
        None
    }
}

pub fn run(input: &str) {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let start = (grid[0].iter().position(|&c| c == b'.').unwrap() as i32, 0);
    let end = (
        grid.last()
            .unwrap()
            .iter()
            .position(|&c| c == b'.')
            .unwrap() as i32,
        (grid.len() - 1) as i32,
    );
    for is_slippery in [true, false] {
        let graph = grid_to_graph(&grid, start, end, is_slippery);
        let max_steps = find_longest_path(&graph, start, end).unwrap();
        println!("{}", max_steps);
    }
}
