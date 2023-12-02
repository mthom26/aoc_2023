use std::fs;

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    // Part 01
    let mut part_01_sum = 0;

    'main: for line in input.lines() {
        let parts: Vec<&str> = line.split(|c| c == ':' || c == ';').collect();
        let mut itr = parts.iter();
        let game = itr.next().unwrap();
        let mut game = game.split(' ');
        game.next();
        let game_id = game.next().unwrap().parse::<usize>().unwrap();

        while let Some(s) = itr.next() {
            let rounds: Vec<&str> = s.split(',').map(|a| a.trim()).collect();
            for round in rounds {
                let mut items = round.split(' ');
                let num = items.next().unwrap().parse::<usize>().unwrap();
                let color = items.next().unwrap();

                if check_max(num, color) {
                    continue 'main;
                }
            }
        }

        // No impossible games encountered so add id to sum
        part_01_sum += game_id;
    }

    println!("Part 01 sum: {}", part_01_sum);
}

// Check if game is impossible
fn check_max(num: usize, color: &str) -> bool {
    let max = match color {
        "red" => MAX_RED,
        "green" => MAX_GREEN,
        "blue" => MAX_BLUE,
        _ => unreachable!(),
    };

    num > max
}
