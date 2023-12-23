use crate::utils::parse_ints;

fn get_support_structure(
    bricks_snapshot: &mut Vec<(i32, i32, i32, i32, i32, i32)>,
) -> (Vec<usize>, Vec<Vec<usize>>) {
    let num_bricks = bricks_snapshot.len();
    let mut num_supporting = vec![0; num_bricks];
    let mut supported_bricks = vec![vec![]; num_bricks];

    let mut bricks_stable: Vec<(i32, i32, i32, i32, i32, i32, usize)> = vec![];
    for (index, brick) in bricks_snapshot.iter_mut().enumerate() {
        let stable_bricks_below: Vec<_> = bricks_stable
            .iter()
            .filter(|&stable| {
                stable.1 < brick.0
                    && !(stable.2 > brick.3
                        || brick.2 > stable.3
                        || stable.4 > brick.5
                        || brick.4 > stable.5)
            })
            .collect();
        let new_z1 = if stable_bricks_below.is_empty() {
            1
        } else {
            let highest_z2 = stable_bricks_below
                .iter()
                .map(|brick| brick.1)
                .max()
                .unwrap();
            let bricks_supporting: Vec<_> = stable_bricks_below
                .iter()
                .filter(|brick| brick.1 == highest_z2)
                .map(|&brick| *brick)
                .collect();
            num_supporting[index] = bricks_supporting.len();
            bricks_supporting
                .iter()
                .for_each(|brick| supported_bricks[brick.6].push(index));
            highest_z2 + 1
        };
        bricks_stable.push((
            new_z1,
            brick.1 - brick.0 + new_z1,
            brick.2,
            brick.3,
            brick.4,
            brick.5,
            index,
        ));
    }
    (num_supporting, supported_bricks)
}

fn count_falling_bricks(
    index: usize,
    num_supporting: &[usize],
    supported_bricks: &[Vec<usize>],
) -> u32 {
    let mut num_supporting = num_supporting.to_vec();
    let mut num_fallen = 0;
    let mut to_remove = vec![index];
    while let Some(removed_index) = to_remove.pop() {
        num_fallen += 1;
        for &brick_above_index in &supported_bricks[removed_index] {
            num_supporting[brick_above_index] -= 1;
            if num_supporting[brick_above_index] == 0 {
                to_remove.push(brick_above_index);
            }
        }
    }
    num_fallen - 1
}

pub fn run(input: &str) {
    let mut bricks_snapshot: Vec<_> = input
        .lines()
        .map(|line| {
            let vals: Vec<i32> = parse_ints(line, false);
            (vals[2], vals[5], vals[0], vals[3], vals[1], vals[4]) // z1-z2, x1-x2, y1-y2
        })
        .collect();
    bricks_snapshot.sort(); // from lowest to highest

    let (num_supporting, supported_bricks) = get_support_structure(&mut bricks_snapshot);

    let num_safe_to_disintegrate = supported_bricks
        .iter()
        .enumerate()
        .filter(|&(_, supported)| supported.iter().all(|&index| num_supporting[index] != 1))
        .count();
    println!("{}", num_safe_to_disintegrate);

    let total_falling_bricks: u32 = (0..bricks_snapshot.len())
        .map(|index| count_falling_bricks(index, &num_supporting, &supported_bricks))
        .sum();
    println!("{}", total_falling_bricks);
}
