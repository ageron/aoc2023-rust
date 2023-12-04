use itertools::Itertools;

pub fn run(input: &str) {
    let lines = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect_vec();

    let score: u32 = lines.iter().sum();
    println!("{}", score);
    let score: u32 = lines.iter().sum();
    println!("{}", score);
}
