pub fn run(input: &str) {
    let data: Vec<_> = input
        .lines()
        .map(|line| line.as_bytes())
        .collect();

    println!("{:?}", data);
}
