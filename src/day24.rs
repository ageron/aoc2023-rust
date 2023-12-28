use crate::utils::parse_int_vecs;

use hashbrown::{HashMap, HashSet};

fn is_right_side(hailstone: &[i64], x: f64, y: f64) -> bool {
    if (hailstone[0] as f64) < x && hailstone[3] <= 0 {
        return false;
    }
    if (hailstone[0] as f64) > x && hailstone[3] >= 0 {
        return false;
    }
    if (hailstone[1] as f64) < y && hailstone[4] <= 0 {
        return false;
    }
    if (hailstone[1] as f64) > y && hailstone[4] >= 0 {
        return false;
    }
    true
}

fn are_paths_intersecting_in_area(h1: &[i64], h2: &[i64], min: f64, max: f64) -> bool {
    assert!(h1[3] != 0 && h2[3] != 0 && h1[4] != 0 && h2[4] != 0);

    if h1[4] * h2[3] == h1[3] * h2[4] {
        if h1[4] * (h2[0] - h1[0]) == h1[3] * (h2[1] - h1[1]) {
            // overlapping paths
            unreachable!();
        } else {
            // parallel paths
            false
        }
    } else {
        let r1 = (h1[4] as f64) / (h1[3] as f64);
        let r2 = (h2[4] as f64) / (h2[3] as f64);
        let x =
            (h2[1] as f64 - h1[1] as f64 - r2 * (h2[0] as f64) + r1 * (h1[0] as f64)) / (r1 - r2);
        let y = h1[1] as f64 + r1 * (x - h1[0] as f64);
        (min..=max).contains(&x)
            && (min..=max).contains(&y)
            && is_right_side(h1, x, y)
            && is_right_side(h2, x, y)
    }
}

fn count_intersecting_paths_in_area(hailstones: &[Vec<i64>], min: f64, max: f64) -> u32 {
    let n = hailstones.len();
    (0..(n - 1))
        .map(|i| {
            ((i + 1)..n)
                .filter(|&j| {
                    are_paths_intersecting_in_area(&hailstones[i], &hailstones[j], min, max)
                })
                .count() as u32
        })
        .sum()
}

fn get_prime_powers(mut n: i64) -> Vec<(i64, i64)> {
    if n <= 1 {
        return vec![];
    }
    let mut p = 2;
    let mut powers = vec![];
    let mut d = 1;
    loop {
        if n % p == 0 {
            d *= p;
            n /= p;
        } else {
            if d > 1 {
                powers.push((p, d));
                d = 1;
            }
            p = if p == 2 { 3 } else { p + 2 };
            if p * p > n {
                p = n;
            }
            if n == 1 {
                break;
            }
        }
    }
    powers
}

fn chinese_theorem(mods: &HashMap<i64, (i64, i64)>) -> i64 {
    let mut n: i64 = 0;
    let mut step = 1;
    let mut mods: Vec<_> = mods.values().collect();
    mods.sort();

    for &(prime_power, modulo) in mods.into_iter().rev() {
        while n.rem_euclid(prime_power) != modulo {
            if n > i64::MAX / 100 {
                step = -step;
            }
            n += step;
        }
        if step.abs() <= i64::MAX / prime_power {
            step *= prime_power; // avoid overflow
        }
    }
    n
}

fn find_stone_start_position_and_speed(hailstones: &[Vec<i64>]) -> Vec<(i64, i64)> {
    let mut stone_x_speed = 0;
    'speed_loop: loop {
        if stone_x_speed > 0 {
            stone_x_speed = -stone_x_speed;
        } else {
            stone_x_speed = -stone_x_speed + 1;
        }
        let mut mods: HashMap<i64, (i64, i64)> = HashMap::new();
        for hailstone in hailstones {
            let hailstone_x0 = hailstone[0];
            let hailstone_x_speed = hailstone[3];
            let x_speed_diff = (stone_x_speed - hailstone_x_speed).abs();
            if x_speed_diff == 0 {
                if let Some(solution) = check_solution(hailstones, hailstone_x0, stone_x_speed) {
                    return solution;
                }
                continue 'speed_loop; // assuming the stone can't have the same speed as any hailstone
            }
            for (prime, prime_power) in get_prime_powers(x_speed_diff) {
                let modulo = hailstone_x0.rem_euclid(prime_power);
                if let Some(&(existing_prime_power, existing_modulo)) = mods.get(&prime) {
                    if prime_power == existing_prime_power && modulo != existing_modulo {
                        continue 'speed_loop;
                    }
                    if prime_power > existing_prime_power {
                        if modulo.rem_euclid(existing_prime_power) != existing_modulo {
                            continue 'speed_loop;
                        }
                        mods.insert(prime, (prime_power, modulo));
                    } else if existing_modulo.rem_euclid(prime_power) != modulo {
                        continue 'speed_loop;
                    }
                } else {
                    mods.insert(prime, (prime_power, modulo));
                }
            }
        }
        let stone_x0 = chinese_theorem(&mods);
        if let Some(solution) = check_solution(hailstones, stone_x0, stone_x_speed) {
            return solution;
        }
    }
}

fn check_solution(
    hailstones: &[Vec<i64>],
    stone_x0: i64,
    stone_x_speed: i64,
) -> Option<Vec<(i64, i64)>> {
    let mut solution = vec![];
    let mut crash_times = vec![];
    for hailstone in hailstones {
        let hailstone_x0 = hailstone[0];
        let hailstone_x_speed = hailstone[3];
        let delta_speed = hailstone_x_speed - stone_x_speed;
        if delta_speed == 0 {
            crash_times.push(None);
        } else {
            let delta_x0 = stone_x0 - hailstone_x0;
            if delta_x0 % delta_speed != 0 {
                return None;
            } else {
                let crash_time = delta_x0 / delta_speed;
                if crash_time < 0 {
                    return None;
                }
                crash_times.push(Some(crash_time));
            }
        }
    }
    let some_crash_times: Vec<_> = crash_times.iter().filter_map(|&x| x).collect();
    let unique_crash_times: HashSet<_> = some_crash_times.iter().collect();
    if unique_crash_times.len() != some_crash_times.len() {
        return None; // crashes into multiple hailstones at the same time
    }
    solution.push((stone_x0, stone_x_speed)); // dimension x looks good!

    let mut t1_t2: Vec<_> = crash_times
        .iter()
        .enumerate()
        .filter_map(|(index, &time)| time.map(|time| (index, time)))
        .take(2)
        .collect();
    t1_t2.sort_by_key(|&(_, time)| time);
    let ((i1, t1), (i2, t2)) = (t1_t2[0], t1_t2[1]);

    for dimension in [1, 2] {
        let hailstone_t1_coord0 = hailstones[i1][dimension];
        let hailstone_t2_coord0 = hailstones[i2][dimension];
        let hailstone_t1_speed = hailstones[i1][dimension + 3];
        let hailstone_t2_speed = hailstones[i2][dimension + 3];
        let distance = hailstone_t2_coord0 + t2 * hailstone_t2_speed
            - hailstone_t1_coord0
            - t1 * hailstone_t1_speed;
        let delta_time = t2 - t1;

        if distance % delta_time != 0 {
            return None;
        }
        let stone_speed = distance / delta_time;
        let stone_coord0 = hailstones[i1][dimension] + t1 * (hailstone_t1_speed - stone_speed);

        for (hailstone, crash_time) in hailstones.iter().zip(crash_times.iter()) {
            if let Some(crash_time) = crash_time {
                if hailstone[dimension] + crash_time * (hailstone[dimension + 3] - stone_speed)
                    != stone_coord0
                {
                    return None;
                }
            }
        }
        solution.push((stone_coord0, stone_speed)); // this dimension looks good too!
    }
    Some(solution)
}

pub fn run(input: &str) {
    let hailstones: Vec<Vec<i64>> = parse_int_vecs(input, true);

    const MIN: f64 = 200000000000000f64;
    const MAX: f64 = 400000000000000f64;
    let num_intersecting_paths_in_area = count_intersecting_paths_in_area(&hailstones, MIN, MAX);
    println!("{}", num_intersecting_paths_in_area);

    let stone_start_position_and_speed = find_stone_start_position_and_speed(&hailstones);
    let sum_of_coordinates: i64 = stone_start_position_and_speed
        .iter()
        .map(|&(coordinate, _)| coordinate)
        .sum();
    println!("{}", sum_of_coordinates);
}
