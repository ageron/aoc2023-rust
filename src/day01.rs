use std::collections::HashMap;

type Digits = HashMap<String, u32>;

fn calibration_value(line: &str, digits: &Digits) -> u32 {
    let first_digit = digits
        .iter()
        .map(|(digit_string, &value)| (line.find(digit_string), value))
        .filter(|(index, _)| index.is_some())
        .min()
        .unwrap()
        .1;
    let last_digit = digits
        .iter()
        .map(|(digit_string, &value)| (line.rfind(digit_string), value))
        .max()
        .unwrap()
        .1;
    first_digit * 10 + last_digit
}

pub fn run(input: &str) {
    // digits 0 to 9
    let digits: HashMap<String, u32> = (0..=9).map(|i| (i.to_string(), i)).collect();

    let sum_of_calibration_values: u32 = input
        .lines()
        .map(|line| calibration_value(line, &digits))
        .sum();

    println!("{sum_of_calibration_values}");

    // digits 0 to 9 plus one, two, ..., nine
    let digits: HashMap<String, u32> = "one,two,three,four,five,six,seven,eight,nine"
        .split(',')
        .map(|s| s.to_string())
        .zip(1..=9)
        .chain(digits)
        .collect();

    let sum_of_calibration_values: u32 = input
        .lines()
        .map(|line| calibration_value(line, &digits))
        .sum();

    println!("{sum_of_calibration_values}");
}
