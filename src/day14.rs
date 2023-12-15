use crate::utils::compute_hash;
use std::collections::HashMap;

fn tilt(platform: &mut Vec<Vec<u8>>, dx: i32, dy: i32) {
    let start_x: usize = if dx == -1 { 1 } else { 0 };
    let start_y: usize = if dy == -1 { 1 } else { 0 };
    let end_x = platform[0].len() - if dx == 1 { 1 } else { 0 };
    let end_y = platform.len() - if dy == 1 { 1 } else { 0 };
    loop {
        let mut num_changes = 0;
        for x in start_x..end_x {
            for y in start_y..end_y {
                let here = platform[y][x];
                if here == b'O' {
                    let (there_x, there_y) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                    let there = platform[there_y][there_x];
                    if there == b'.' {
                        platform[y][x] = b'.';
                        platform[there_y][there_x] = b'O';
                        num_changes += 1;
                    }
                }
            }
        }
        if num_changes == 0 {
            break;
        }
    }
}

fn compute_total_load(platform: &[Vec<u8>]) -> u32 {
    platform
        .iter()
        .enumerate()
        .map(|(y, row)| {
            let num_rounded_rocks = row.iter().filter(|&&c| c == b'O').count();
            (num_rounded_rocks * (platform.len() - y)) as u32
        })
        .sum()
}

fn cycle(platform: &mut Vec<Vec<u8>>) {
    for (dx, dy) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
        tilt(platform, dx, dy);
    }
}

fn repeat_cycles(platform: &mut Vec<Vec<u8>>, num_cycles: u32) {
    let mut hash_index = HashMap::new();
    for index in 0.. {
        let hash = compute_hash(platform);
        let hash_index = *hash_index.entry(hash).or_insert(index);
        if hash_index < index {
            let modulo = index - hash_index;
            let remaining_cycles = (num_cycles - index) % modulo;
            (0..remaining_cycles).for_each(|_| cycle(platform));
            return;
        }
        cycle(platform);
    }
    unreachable!()
}

pub fn run(input: &str) {
    let mut platform: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();

    tilt(&mut platform, 0, -1);
    let total_load = compute_total_load(&platform);
    println!("{}", total_load);

    repeat_cycles(&mut platform, 1_000_000_000);
    let total_load = compute_total_load(&platform);
    println!("{}", total_load);
}
