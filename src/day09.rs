use crate::utils::parse_int_vecs;

fn extrapolate(readings: &[i64], is_future: bool) -> i64 {
    if readings.iter().all(|&r| r == 0) {
        return 0;
    }
    let differences: Vec<_> = readings[1..]
        .iter()
        .zip(readings[..(readings.len() - 1)].iter())
        .map(|(a, b)| a - b)
        .collect();
    if is_future {
        readings.last().unwrap() + extrapolate(&differences, is_future)
    } else {
        readings.first().unwrap() - extrapolate(&differences, is_future)
    }
}

pub fn run(input: &str) {
    let data: Vec<Vec<i64>> = parse_int_vecs(input, true);
    for is_future in [true, false] {
        let sum_of_extrapolated_values: i64 = data
            .iter()
            .map(|readings| extrapolate(readings, is_future))
            .sum();
        println!("{}", sum_of_extrapolated_values);
    }
}
