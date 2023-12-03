use itertools::Itertools;

/// Parses a line and returns the game id and the max number of balls per color
fn parse_line(line: &str) -> (u32, [u32; 3]) {
    let mut parts = line.split(':');
    let header = parts.next().expect("Invalid line format (header)");
    let game_id: u32 = header["Game ".len()..].parse().expect("Invalid line format (game id)");
    let sets = parts.next().expect("Invalid line format (sets)");
    let mut maxs: [u32; 3] = [0, 0, 0];
    for n_color in sets.split(|c| c == ';' || c == ',') {
        let mut set_parts = n_color[1..].split_whitespace();
        let number: u32 = set_parts.next().expect("Invalid line format (no number)").parse().expect("Invalid line format (number format)");
        let color = set_parts.next().expect("Invalid line format (no color)");
        let index = match color {
            "red" => 0,
            "green" => 1,
            "blue" => 2,
            _ => unreachable!()
        };
        maxs[index] = maxs[index].max(number);
    }
    (game_id, maxs)
}

fn is_possible(maxs: &[u32; 3]) -> bool {
    maxs[0] <= 12 && maxs[1] <= 13 && maxs[2] <= 14
}

fn power(maxs: &[u32; 3]) -> u32 {
    maxs[0] * maxs[1] * maxs[2]
}

pub fn run(input: &str) {
    let all_games = input
        .lines()
        .map(parse_line)
        .collect_vec();

    let sum_of_possible_game_ids: u32 = all_games.iter().filter(|(_, maxs)| is_possible(maxs)).map(|(game_id, _)| game_id).sum();
    println!("{}", sum_of_possible_game_ids);

    let sum_of_game_powers: u32 = all_games.iter().map(|(_, maxs)| power(maxs)).sum();
    println!("{}", sum_of_game_powers);
}
