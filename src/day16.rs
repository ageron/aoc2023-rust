fn propagate_light_beam(
    grid: &[&[u8]],
    mut x: i32,
    mut y: i32,
    mut dx: i32,
    mut dy: i32,
    visited_cells: &mut Vec<Vec<u8>>,
) {
    let size = grid.len() as i32;
    while x >= 0 && x < size && y >= 0 && y < size {
        let direction = match (dx, dy) {
            (0, 1) => 1,
            (0, -1) => 2,
            (1, 0) => 4,
            (-1, 0) => 8,
            _ => unreachable!(),
        };
        if visited_cells[y as usize][x as usize] & direction != 0 {
            break;
        }
        visited_cells[y as usize][x as usize] |= direction;
        let c = grid[y as usize][x as usize];
        match c {
            b'.' => {}
            b'|' => {
                if dx != 0 {
                    dx = 0;
                    dy = 1;
                    propagate_light_beam(grid, x, y - 1, dx, -dy, visited_cells);
                }
            }
            b'-' => {
                if dy != 0 {
                    dx = 1;
                    dy = 0;
                    propagate_light_beam(grid, x - 1, y, -dx, dy, visited_cells);
                }
            }
            b'\\' => {
                if dx == 0 {
                    dx = dy;
                    dy = 0;
                } else {
                    dy = dx;
                    dx = 0;
                }
            }
            b'/' => {
                if dx == 0 {
                    dx = -dy;
                    dy = 0;
                } else {
                    dy = -dx;
                    dx = 0;
                }
            }
            _ => unreachable!(),
        }
        x += dx;
        y += dy;
    }
}

fn count_energized_cells(grid: &[&[u8]], x: i32, y: i32, dx: i32, dy: i32) -> usize {
    let mut visited_cells = vec![vec![0u8; grid[0].len()]; grid.len()];
    propagate_light_beam(grid, x, y, dx, dy, &mut visited_cells);
    visited_cells
        .iter()
        .map(|row| row.iter().filter(|&&c| c != 0).count())
        .sum()
}

pub fn run(input: &str) {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let num_energized_cells = count_energized_cells(&grid, 0, 0, 1, 0);
    println!("{:?}", num_energized_cells);

    let size = grid.len() as i32;
    let max_energized_cells = (0..size)
        .flat_map(|i| {
            [
                (i, 0, 0, 1),
                (i, size - 1, 0, -1),
                (0, i, 1, 0),
                (size - 1, i, -1, 0),
            ]
        })
        .map(|(x, y, dx, dy)| count_energized_cells(&grid, x, y, dx, dy))
        .max()
        .unwrap();
    println!("{}", max_energized_cells);
}
