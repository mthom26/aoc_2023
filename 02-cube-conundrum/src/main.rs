use std::fs;

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut part_01_sum = 0;
    let mut part_02_sum = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(|c| c == ':' || c == ';').collect();
        let mut itr = parts.iter();
        let game = itr.next().unwrap();
        let mut game = game.split(' ');
        game.next();
        let game_id = game.next().unwrap().parse::<usize>().unwrap();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        let mut is_game_impossible = false;

        while let Some(s) = itr.next() {
            let rounds: Vec<&str> = s.split(',').map(|a| a.trim()).collect();
            for round in rounds {
                let mut items = round.split(' ');
                let num = items.next().unwrap().parse::<usize>().unwrap();
                let color = items.next().unwrap();

                match color {
                    "red" => {
                        if num > max_red {
                            max_red = num;
                        }
                    }
                    "green" => {
                        if num > max_green {
                            max_green = num;
                        }
                    }
                    "blue" => {
                        if num > max_blue {
                            max_blue = num;
                        }
                    }
                    _ => unreachable!(),
                }

                if check_max(num, color) {
                    is_game_impossible = true;
                }
            }
        }

        if !is_game_impossible {
            part_01_sum += game_id;
        }
        part_02_sum += max_red * max_green * max_blue;
    }

    println!("Part 01 sum: {}", part_01_sum);
    println!("Part 02 sum: {}", part_02_sum);
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
