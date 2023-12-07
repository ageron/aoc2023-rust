use crate::utils::parse_int_vecs;

fn number_of_ways_to_beat_record(race_time: i64, record_distance: i64) -> i64 {
    // To beat the record distance, we need:
    // push_time * (race_time - push_time) ≥ record_distance + 1
    // Solving this mathematically, we get:
    let discriminant = ((race_time * race_time - 4 * (record_distance + 1)) as f64).sqrt();
    let min_time = ((race_time as f64 - discriminant) / 2.0).ceil() as i64;
    let max_time = ((race_time as f64 + discriminant) / 2.0).floor() as i64;
    max_time - min_time + 1
}

fn parse_number_ignoring_whitespaces(line: &str) -> i64 {
    line.split(':')
        .nth(1)
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap()
}

pub fn run(input: &str) {
    let numbers: Vec<Vec<i64>> = parse_int_vecs(input, false);
    let races: Vec<(i64, i64)> = numbers[0]
        .iter()
        .zip(numbers[1].iter())
        .map(|(&a, &b)| (a, b))
        .collect();
    let n_ways = races
        .iter()
        .map(|(race_time, record_distance)| {
            number_of_ways_to_beat_record(*race_time, *record_distance)
        })
        .reduce(|a, b| a * b)
        .unwrap();
    println!("{}", n_ways);

    let mut lines = input.lines();
    let race_time = parse_number_ignoring_whitespaces(lines.next().unwrap());
    let record_distance = parse_number_ignoring_whitespaces(lines.next().unwrap());

    let n_ways = number_of_ways_to_beat_record(race_time, record_distance);
    println!("{}", n_ways);
}
