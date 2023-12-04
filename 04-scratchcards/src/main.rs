use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut part_one_sum = 0;

    for line in input.lines() {
        let (_, cards) = line.split_once(':').unwrap();
        let (winning_cards, current_cards) = cards
            .split_once('|')
            .map(|(a, b)| (a.trim(), b.trim()))
            .unwrap();

        let mut winners: usize = 0;
        for card in winning_cards.split(' ').filter(|c| !c.is_empty()) {
            for current_card in current_cards.split(' ').filter(|c| !c.is_empty()) {
                if current_card == card {
                    match winners {
                        0 => winners += 1,
                        _ => winners *= 2,
                    }
                }
            }
        }
        part_one_sum += winners;
    }

    println!("Part One sum: {}", part_one_sum);
}
