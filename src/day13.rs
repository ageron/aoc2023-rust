fn is_reflected(
    grid: &[Vec<bool>],
    mirror_index: usize,
    is_horizontal: bool,
    flipped_pos: (usize, usize),
) -> bool {
    let (size_along_mirror, size_perpendicular) = if is_horizontal {
        (grid[0].len(), grid.len())
    } else {
        (grid.len(), grid[0].len())
    };
    let num_reflected = (mirror_index + 1).min(size_perpendicular - mirror_index - 1);
    for i in 0..size_along_mirror {
        for j in 0..num_reflected {
            let (pos, pos_reflected) = if is_horizontal {
                ((i, mirror_index - j), (i, mirror_index + j + 1))
            } else {
                ((mirror_index - j, i), (mirror_index + j + 1, i))
            };
            if (grid[pos.1][pos.0] != grid[pos_reflected.1][pos_reflected.0]) ^ (pos == flipped_pos)
            {
                return false;
            }
        }
    }
    true
}

fn find_reflection_id(
    grid: &[Vec<bool>],
    flipped_pos: (usize, usize),
    ignored_reflection_id: usize,
) -> usize {
    let mut reflection_id = 0;
    for (is_horizontal, multiplier, size_perpendicular) in
        [(false, 1, grid[0].len()), (true, 100, grid.len())]
    {
        for mirror_index in 0..size_perpendicular - 1 {
            if is_reflected(grid, mirror_index, is_horizontal, flipped_pos) {
                let new_reflection_id = (mirror_index + 1) * multiplier;
                if ignored_reflection_id == 0 {
                    return new_reflection_id;
                }
                if new_reflection_id != ignored_reflection_id {
                    if reflection_id != 0 {
                        return 0;
                    }
                    reflection_id = new_reflection_id;
                }
            }
        }
    }
    reflection_id
}

fn smudge_reflection_id(grid: &[Vec<bool>], ignored_reflection_id: usize) -> usize {
    for flipped_col in 0..grid[0].len() {
        for flipped_row in 0..grid.len() {
            let reflection_id =
                find_reflection_id(grid, (flipped_col, flipped_row), ignored_reflection_id);
            if reflection_id != 0 {
                return reflection_id;
            }
        }
    }
    unreachable!();
}

pub fn run(input: &str) {
    let grids: Vec<Vec<Vec<bool>>> = input
        .split("\n\n")
        .map(|grid| {
            grid.lines()
                .map(|line| line.bytes().map(|c| c == b'#').collect())
                .collect()
        })
        .collect();

    let reflection_ids: Vec<_> = grids
        .iter()
        .map(|grid| find_reflection_id(grid, (grid[0].len(), 0), 0))
        .collect();
    let total_reflection_ids: usize = reflection_ids.iter().sum();
    println!("{}", total_reflection_ids);

    let total_smudge_reflection_ids: usize = grids
        .iter()
        .zip(reflection_ids.iter())
        .map(|(grid, &ignored_reflection_id)| smudge_reflection_id(grid, ignored_reflection_id))
        .sum();
    println!("{}", total_smudge_reflection_ids);
}
