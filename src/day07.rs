use crate::utils::argmax;

fn card_rank(card: char, with_jokers: bool) -> usize {
    let order = if with_jokers {
        "J23456789TQKA"
    } else {
        "23456789TJQKA"
    };
    order.chars().position(|c| c == card).unwrap()
}

fn hand_type(hand: &str, with_jokers: bool) -> i64 {
    let mut card_counts = [0i64; 13];
    for c in hand.chars() {
        card_counts[card_rank(c, with_jokers)] += 1;
    }
    let mut n_jokers = 0;
    if with_jokers {
        n_jokers = card_counts[0];
        card_counts[0] = 0;
    }
    let rank_of_most_frequent = argmax(&card_counts).unwrap();
    if with_jokers {
        card_counts[rank_of_most_frequent] += n_jokers;
    }
    let n_pairs = card_counts.into_iter().filter(|&c| c == 2).count() as i64;
    match card_counts[rank_of_most_frequent] {
        5 => 6,
        4 => 5,
        3 => 3 + n_pairs,
        2 => n_pairs,
        1 => 0,
        _ => unreachable!(),
    }
}

fn compare_hands(h1: &str, h2: &str, with_jokers: bool) -> std::cmp::Ordering {
    let type_cmp = hand_type(h1, with_jokers).cmp(&hand_type(h2, with_jokers));
    if type_cmp != std::cmp::Ordering::Equal {
        return type_cmp;
    }
    for (c1, c2) in h1.chars().zip(h2.chars()) {
        if c1 != c2 {
            return card_rank(c1, with_jokers).cmp(&card_rank(c2, with_jokers));
        }
    }
    unreachable!(); // std::cmp::Ordering::Equal
}

fn sort_hands_and_get_total_winnings(hands: &mut [(String, i64)], with_jokers: bool) -> i64 {
    hands.sort_by(|(h1, _), (h2, _)| compare_hands(h1, h2, with_jokers));
    hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as i64 + 1) * bid)
        .sum()
}

pub fn run(input: &str) {
    let mut hands: Vec<(String, i64)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            (
                parts.next().unwrap().to_string(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    for with_jokers in [false, true] {
        let total_winnings = sort_hands_and_get_total_winnings(&mut hands, with_jokers);
        println!("{}", total_winnings);
    }
}
