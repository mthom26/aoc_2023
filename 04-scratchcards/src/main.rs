use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let l = input.lines().count();
    let mut part_one_sum = 0;
    let mut cards_totals = vec![1usize; l];

    for (i, line) in input.lines().enumerate() {
        let (_, cards) = line.split_once(':').unwrap();
        let (winning_cards, current_cards) = cards
            .split_once('|')
            .map(|(a, b)| (a.trim(), b.trim()))
            .unwrap();

        let mut winners: usize = 0;
        let mut matching_cards: usize = 0;
        for card in winning_cards.split(' ').filter(|c| !c.is_empty()) {
            for current_card in current_cards.split(' ').filter(|c| !c.is_empty()) {
                if current_card == card {
                    matching_cards += 1;
                    match winners {
                        0 => winners += 1,
                        _ => winners *= 2,
                    }
                }
            }
        }

        part_one_sum += winners;
        // Number of duplicates the current card
        let duplicates = cards_totals[i];
        for idx in 1..=matching_cards {
            cards_totals[idx + i] += duplicates;
        }
    }

    println!("Part One sum: {}", part_one_sum);

    let part_two_sum: usize = cards_totals.iter().sum();
    println!("Part Two sum: {}", part_two_sum);
}
