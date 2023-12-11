use std::collections::HashSet;

fn expanded_distance(
    coord1: i32,
    coord2: i32,
    galaxy_coords: &HashSet<i32>,
    expansion_factor: u64,
) -> u64 {
    let min_coord = coord1.min(coord2);
    let max_coord = coord1.max(coord2);
    let distance = (max_coord - min_coord) as u64;
    if distance < 2 {
        return distance;
    }
    let n_galaxies_in_between = galaxy_coords
        .iter()
        .filter(|&&coord| coord > min_coord && coord < max_coord)
        .count() as u64;
    (distance - 1 - n_galaxies_in_between) * expansion_factor + n_galaxies_in_between + 1
}

fn shortest_distance(
    g1: (i32, i32),
    g2: (i32, i32),
    galaxy_cols: &HashSet<i32>,
    galaxy_rows: &HashSet<i32>,
    expansion_factor: u64,
) -> u64 {
    expanded_distance(g1.0, g2.0, galaxy_cols, expansion_factor)
        + expanded_distance(g1.1, g2.1, galaxy_rows, expansion_factor)
}

fn sum_of_shortest_distances(galaxies: &[(i32, i32)], expansion_factor: u64) -> u64 {
    let galaxy_cols: HashSet<i32> = galaxies.iter().map(|&(x, _)| x).collect();
    let galaxy_rows: HashSet<i32> = galaxies.iter().map(|&(_, y)| y).collect();
    let mut total_distance = 0;
    for i in 0..(galaxies.len() - 1) {
        for j in (i + 1)..galaxies.len() {
            total_distance += shortest_distance(
                galaxies[i],
                galaxies[j],
                &galaxy_cols,
                &galaxy_rows,
                expansion_factor,
            )
        }
    }
    total_distance
}

pub fn run(input: &str) {
    let galaxies: Vec<(i32, i32)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|&(_, c)| c == b'#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect();
    for expansion_factor in [2, 1000000] {
        let total_distance = sum_of_shortest_distances(&galaxies, expansion_factor);
        println!("{}", total_distance);
    }
}
