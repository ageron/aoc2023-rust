fn parse_ints(s: &str) -> Vec<u32> {
    s.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn n_matching(winning_numbers: &[u32], our_numbers: &[u32]) -> usize {
    winning_numbers
        .iter()
        .filter(|w| our_numbers.contains(w))
        .count()
}

pub fn run(input: &str) {
    let scratch_cards: Vec<_> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ").nth(1).unwrap().split(" | ");
            let winning_numbers: Vec<u32> = parse_ints(parts.next().unwrap());
            let our_numbers: Vec<u32> = parse_ints(parts.next().unwrap());
            (winning_numbers, our_numbers)
        })
        .collect();

    let total_points: u32 = scratch_cards
        .iter()
        .map(|(winning_numbers, our_numbers)| {
            u32::pow(2, n_matching(winning_numbers, our_numbers) as u32) / 2
        })
        .sum();
    println!("{}", total_points);

    let mut n_cards = vec![1; scratch_cards.len()];
    for (i, (winning_numbers, our_numbers)) in scratch_cards.iter().enumerate() {
        let final_index = scratch_cards
            .len()
            .min(i + 1 + n_matching(winning_numbers, our_numbers));
        for j in i + 1..final_index {
            n_cards[j] += n_cards[i]
        }
    }
    let total_n_cards: u32 = n_cards.iter().sum();
    println!("{}", total_n_cards);
}
