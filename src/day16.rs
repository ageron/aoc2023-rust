use std::collections::HashSet;

fn count_energized_cells(
    grid: &[&[u8]],
    mut x: i32,
    mut y: i32,
    mut dx: i32,
    mut dy: i32,
    visited_cells: &mut HashSet<(i32, i32, i32, i32)>,
    energized_cells: &mut HashSet<(i32, i32)>,
) -> usize {
    // println!("{},{} {},{}", x, y, dx, dy);
    let size = grid.len() as i32;
    while x >= 0 && x < size && y >= 0 && y < size && !visited_cells.contains(&(x, y, dx, dy)) {
        visited_cells.insert((x, y, dx, dy));
        energized_cells.insert((x, y));
        let c = grid[y as usize][x as usize];
        match c {
            b'.' => {}
            b'|' => {
                if dx != 0 {
                    dx = 0;
                    dy = 1;
                    count_energized_cells(grid, x, y - 1, dx, -dy, visited_cells, energized_cells);
                }
            }
            b'-' => {
                if dy != 0 {
                    dx = 1;
                    dy = 0;
                    count_energized_cells(grid, x - 1, y, -dx, dy, visited_cells, energized_cells);
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
    energized_cells.len()
}

pub fn run(input: &str) {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let num_energized_cells =
        count_energized_cells(&grid, 0, 0, 1, 0, &mut HashSet::new(), &mut HashSet::new());
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
        .map(|(x, y, dx, dy)| {
            count_energized_cells(
                &grid,
                x,
                y,
                dx,
                dy,
                &mut HashSet::new(),
                &mut HashSet::new(),
            )
        })
        .max()
        .unwrap();
    println!("{}", max_energized_cells);
}
