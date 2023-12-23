use hashbrown::HashMap;

use std::collections::VecDeque;

fn find_start_plot(garden: &[&[u8]]) -> (i32, i32) {
    for (y, row) in garden.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == b'S' {
                return (x as i32, y as i32);
            }
        }
    }
    unreachable!()
}

fn get_visited_plots(
    garden: &[&[u8]],
    num_steps: u64,
    is_infinite: bool,
) -> HashMap<(i32, i32), u64> {
    let height = garden.len() as i32;
    let width = garden[0].len() as i32;
    let (start_x, start_y) = find_start_plot(garden);
    let mut to_visit = VecDeque::from([(start_x, start_y, 0)]);
    let mut visited = HashMap::new();
    while !to_visit.is_empty() {
        let (x, y, steps) = to_visit.pop_front().unwrap();
        if !is_infinite && (x < 0 || x >= width || y < 0 || y >= height) {
            continue;
        }
        let c = if is_infinite {
            garden[y.rem_euclid(height) as usize][x.rem_euclid(width) as usize]
        } else {
            garden[y as usize][x as usize]
        };
        if c != b'.' && c != b'S' {
            continue;
        }
        if visited.contains_key(&(x, y)) {
            continue;
        }
        visited.insert((x, y), steps);
        if steps < num_steps {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                to_visit.push_back((x + dx, y + dy, steps + 1))
            }
        }
    }
    visited
}

fn count_reachable_plots(garden: &[&[u8]], num_steps: u64, is_infinite: bool) -> u64 {
    get_visited_plots(garden, num_steps, is_infinite)
        .values()
        .filter(|&&v| v % 2 == if is_infinite { 1 } else { 0 })
        .count() as u64
}

pub fn run(input: &str) {
    let garden: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let num_reachable_plots = count_reachable_plots(&garden, 64, false);
    println!("{}", num_reachable_plots);

    let size = garden.len() as u64;
    assert!(garden[0].len() as u64 == size); // the garden must be a square
    const NUM_STEPS: u64 = 26501365;

    let num_visited: Vec<_> = (0..3)
        .map(|n| {
            let num_steps = 2 * size * n + NUM_STEPS % (2 * size);
            count_reachable_plots(&garden, num_steps, true)
        })
        .collect();

    let max_reachable_per_2x2_garden = (num_visited[0] + num_visited[2] - 2 * num_visited[1]) / 4;
    let num_full_2x2_gardens = NUM_STEPS / (2 * size);
    let base_plots = max_reachable_per_2x2_garden - num_visited[0];
    let extra_plots = 4 * max_reachable_per_2x2_garden + num_visited[0] - num_visited[1];

    let total_plots =
        (2 * num_full_2x2_gardens * num_full_2x2_gardens + 2 * num_full_2x2_gardens + 1)
            * max_reachable_per_2x2_garden
            - base_plots
            - extra_plots * num_full_2x2_gardens;
    println!("{:?}", total_plots);
}
