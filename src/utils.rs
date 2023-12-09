use itertools::Itertools;
use num::{Integer, Num};
use std::str::FromStr;

pub fn parse_ints<T: Num + FromStr>(input: &str, signed: bool) -> Vec<T> {
    input
        .split(|c: char| !(c.is_ascii_digit() || (signed && c == '-')))
        .filter_map(|s| s.parse().ok())
        .collect()
}

pub fn parse_int_vecs<T: Num + FromStr>(input: &str, signed: bool) -> Vec<Vec<T>> {
    input
        .lines()
        .map(|line| parse_ints(line, signed))
        .collect_vec()
}

pub fn argmax<T: Ord>(array: &[T]) -> Option<usize> {
    array
        .iter()
        .enumerate()
        .max_by_key(|&(_, count)| count)
        .map(|(index, _)| index)
}

pub fn gcd<T: Integer + Copy>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lcm<T: Integer + Copy>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}
