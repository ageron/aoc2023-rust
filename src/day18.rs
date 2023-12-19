use std::collections::HashSet;

fn get_vertical_trenches_and_ys(dig_plan: &[(u8, i32)]) -> (Vec<(i32, i32, i32)>, Vec<i32>) {
    let mut vertical_trenches = vec![];
    let mut x = 0;
    let mut y = 0;
    let mut ys = HashSet::from([y]);
    for &(direction, num_steps) in dig_plan {
        let previous_y = y;
        match direction {
            b'R' => {
                x += num_steps;
            }
            b'L' => {
                x -= num_steps;
            }
            b'D' => {
                y += num_steps;
                vertical_trenches.push((x, previous_y, y));
            }
            b'U' => {
                y -= num_steps;
                vertical_trenches.push((x, y, previous_y));
            }
            _ => unreachable!(),
        };
        ys.insert(y);
    }
    assert!(x == 0 && y == 0); // ensure this is a loop (I assume trenches don't cross)
    vertical_trenches.sort();
    let mut ys = Vec::from_iter(ys);
    ys.sort();
    (vertical_trenches, ys)
}

fn compute_lagoon_volume(dig_plan: &[(u8, i32)]) -> i64 {
    let (vertical_trenches, ys) = get_vertical_trenches_and_ys(dig_plan);
    ys.iter()
        .map(|&y| y..y + 1)
        .chain(
            ys.iter()
                .zip(ys.iter().skip(1))
                .map(|(&y, &next_y)| y + 1..next_y),
        )
        .map(|y_range| {
            let mut width_inside: i64 = 0;
            let mut is_odd_crossing = false;
            let mut is_border = false;
            let mut is_inside = false;
            for &(x, y1, y2) in &vertical_trenches {
                if y1 > y_range.start || y2 < y_range.end - 1 {
                    continue;
                }
                if y2 > y_range.start {
                    is_odd_crossing = !is_odd_crossing;
                }
                is_border = !is_border && (y1 == y_range.start || y2 == y_range.start);
                if is_inside {
                    if !(is_odd_crossing || is_border) {
                        is_inside = false;
                        width_inside += (x + 1) as i64;
                    }
                } else if is_odd_crossing || is_border {
                    is_inside = true;
                    width_inside -= x as i64;
                }
            }
            y_range.len() as i64 * width_inside
        })
        .sum()
}

fn parse_dig_plan_1(line: &str) -> (u8, i32) {
    let direction = line.as_bytes()[0];
    let num_steps: i32 = line
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    (direction, num_steps)
}

fn parse_dig_plan_2(line: &str) -> (u8, i32) {
    let hex: i32 = line.bytes().fold(0, |hex, b| {
        match b {
            b'0'..=b'9' => hex * 16 + (b - b'0') as i32,
            b'a'..=b'f' => hex * 16 + 10 + (b - b'a') as i32,
            b'#' => 0, // ignore anything before #
            _ => hex,  // ignore other characters
        }
    });
    (b"RDLU"[(hex % 16) as usize], hex / 16)
}

pub fn run(input: &str) {
    for parse_fn in [parse_dig_plan_1, parse_dig_plan_2] {
        let dig_plan: Vec<_> = input.lines().map(parse_fn).collect();
        let lagoon_volume = compute_lagoon_volume(&dig_plan);
        println!("{}", lagoon_volume);
    }
}
