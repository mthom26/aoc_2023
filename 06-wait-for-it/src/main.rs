use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut races = vec![];

    let mut lines = input.lines();
    let (_, times) = lines.next().unwrap().split_once(':').unwrap();
    let (_, distances) = lines.next().unwrap().split_once(':').unwrap();

    for race in times
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .zip(distances.trim().split(' ').filter(|s| !s.is_empty()))
    {
        races.push((
            race.0.parse::<usize>().unwrap(),
            race.1.parse::<usize>().unwrap(),
        ));
    }

    let mut margin = 1;

    for (time, record) in races {
        let mut won_races = 0;
        for s in 1..time {
            let d = s * (time - s);
            if d > record {
                won_races += 1;
            }
        }
        margin *= won_races;
    }

    println!("Part 01: {}", margin);
}
