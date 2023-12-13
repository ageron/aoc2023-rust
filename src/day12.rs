use crate::utils::parse_ints;

fn min_size(group_lengths: &[usize]) -> usize {
    group_lengths.iter().map(|len| len + 1).sum::<usize>()
}

fn count_arrangements(spring_states: &[u8], group_lengths: &[usize]) -> u64 {
    // For every possible position of the center group of damaged springs, count the
    // number of possible arrangements for the left and right groups using recursive
    // calls, and add their product to the total number of arrangements.

    if group_lengths.is_empty() {
        if spring_states.contains(&b'#') {
            return 0;
        } else {
            return 1;
        }
    }
    let center_group_index = group_lengths.len() / 2;
    let center_group_length = group_lengths[center_group_index];
    let left_group_lengths = &group_lengths[..center_group_index];
    let right_group_lengths = &group_lengths[center_group_index + 1..];
    let mut num_arrangements = 0;
    for start_index in min_size(left_group_lengths)
        ..=spring_states.len() - min_size(right_group_lengths) - center_group_length
    {
        if start_index == 0 || b".?".contains(&spring_states[start_index - 1]) {
            let after_index = start_index + center_group_length;
            if !spring_states[start_index..after_index].contains(&b'.')
                && (after_index == spring_states.len()
                    || b".?".contains(&spring_states[after_index]))
            {
                let left_arrangements = count_arrangements(
                    &spring_states[..start_index.max(1) - 1],
                    left_group_lengths,
                );
                if left_arrangements > 0 {
                    let right_start_index = (after_index + 1).min(spring_states.len());
                    let right_arrangements = count_arrangements(
                        &spring_states[right_start_index..],
                        right_group_lengths,
                    );
                    num_arrangements += left_arrangements * right_arrangements;
                }
            }
        }
    }
    num_arrangements
}

pub fn run(input: &str) {
    let condition_records: Vec<_> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let spring_states: &[u8] = parts.next().unwrap().as_bytes();
            let group_lengths: Vec<usize> = parse_ints(parts.next().unwrap(), false);
            (spring_states, group_lengths)
        })
        .collect();

    let num_arrangements: u64 = condition_records
        .iter()
        .map(|(spring_states, group_lengths)| count_arrangements(spring_states, group_lengths))
        .sum();
    println!("{}", num_arrangements);

    let unfolded_condition_records =
        condition_records
            .iter()
            .map(|(spring_states, group_lengths)| {
                let unfolded_spring_states =
                    [std::str::from_utf8(spring_states).unwrap(); 5].join("?");
                let unfolded_group_lengths = group_lengths.repeat(5);
                (unfolded_spring_states, unfolded_group_lengths)
            });
    let num_arrangements: u64 = unfolded_condition_records
        .map(|(spring_states, group_lengths)| {
            count_arrangements(spring_states.as_bytes(), &group_lengths)
        })
        .sum();
    println!("{}", num_arrangements);
}
