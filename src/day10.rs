use std::collections::HashMap;

fn get_start_position(pipes: &[&[u8]]) -> (i32, i32) {
    for (y, row) in pipes.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == b'S' {
                return (x as i32, y as i32);
            }
        }
    }
    unreachable!()
}

fn get_main_loop(
    pipes: &[&[u8]],
    width: i32,
    height: i32,
    start_x: i32,
    start_y: i32,
    mut dx: i32,
    mut dy: i32,
) -> Option<HashMap<(i32, i32), u8>> {
    let mut x = start_x;
    let mut y = start_y;
    let start_dx = dx;
    let start_dy = dy;
    let mut main_loop: HashMap<(i32, i32), u8> = HashMap::new();
    loop {
        x += dx;
        y += dy;
        if x < 0 || x >= width || y < 0 || y >= height {
            return None;
        }
        let c = pipes[y as usize][x as usize];
        if c == b'S' {
            let c = match (start_dx, start_dy, dx, dy) {
                (1, 0, 1, 0) => b'-',
                (1, 0, 0, 1) => b'L',
                (1, 0, 0, -1) => b'F',
                (0, 1, 1, 0) => b'7',
                (0, 1, 0, 1) => b'|',
                (-1, 0, 0, 1) => b'J',
                _ => unreachable!(),
            };
            main_loop.insert((x, y), c);
            return Some(main_loop);
        }
        main_loop.insert((x, y), c);
        (dx, dy) = match (dx, dy, c) {
            (1, 0, b'-') | (0, 1, b'L') | (0, -1, b'F') => (1, 0),
            (1, 0, b'7') | (0, 1, b'|') | (-1, 0, b'F') => (0, 1),
            (-1, 0, b'-') | (0, 1, b'J') | (0, -1, b'7') => (-1, 0),
            (1, 0, b'J') | (-1, 0, b'L') | (0, -1, b'|') => (0, -1),
            _ => {
                return None;
            }
        };
    }
}

fn count_tiles_inside(main_loop: &HashMap<(i32, i32), u8>, width: i32, height: i32) -> u32 {
    let mut num_tiles_inside = 0;
    for y in 0..height {
        let mut is_inside = false;
        for x in 0..width {
            let c = main_loop.get(&(x, y));
            if let Some(&c) = c {
                if c == b'|' || c == b'J' || c == b'L' {
                    is_inside = !is_inside;
                }
            } else if is_inside {
                num_tiles_inside += 1;
            }
        }
    }
    num_tiles_inside
}

pub fn run(input: &str) {
    let pipes: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let width = pipes[0].len() as i32;
    let height = pipes.len() as i32;
    let (start_x, start_y) = get_start_position(&pipes);
    let main_loop = [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .iter()
        .filter_map(|&(dx, dy)| get_main_loop(&pipes, width, height, start_x, start_y, dx, dy))
        .next()
        .unwrap();
    println!("{}", main_loop.len() / 2);
    let num_tiles_inside = count_tiles_inside(&main_loop, width, height);
    println!("{}", num_tiles_inside);
}
