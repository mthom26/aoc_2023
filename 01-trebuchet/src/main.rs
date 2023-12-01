use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut sum: usize = 0;

    for line in input.lines() {
        let filtered: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
        let mut num = String::new();

        if filtered.len() == 0 {
            panic!("No numeric chars");
        }

        if filtered.len() == 1 {
            num.push(filtered[0]);
            num.push(filtered[0]);
        } else {
            num.push(filtered[0]);
            num.push('0');
            for c in filtered.iter().skip(1) {
                num.pop();
                num.push(*c);
            }
        }

        let num = num.parse::<usize>().unwrap();
        sum += num;
    }

    println!("Part 01 Sum: {}", sum);

    // Part 02
    let mut sum_part_02 = 0;

    let patterns = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    for line in input.lines() {
        let mut pairs_first: Vec<(usize, &str)> = Vec::new();
        let mut pairs_last: Vec<(usize, &str)> = Vec::new();

        for p in patterns {
            if let Some(i) = line.find(p) {
                pairs_first.push((i, p));
            }
            if let Some(i) = line.rfind(p) {
                pairs_last.push((i, p));
            }
        }

        pairs_first.sort();
        pairs_last.sort();
        pairs_last.reverse();

        let mut num = String::new();
        // Ugly but works
        match pairs_first[0].1 {
            s if s.len() == 1 => num.push_str(s),
            "one" => num.push('1'),
            "two" => num.push('2'),
            "three" => num.push('3'),
            "four" => num.push('4'),
            "five" => num.push('5'),
            "six" => num.push('6'),
            "seven" => num.push('7'),
            "eight" => num.push('8'),
            "nine" => num.push('9'),
            _ => panic!("This shouldn't happen"),
        }
        match pairs_last[0].1 {
            s if s.len() == 1 => num.push_str(s),
            "one" => num.push('1'),
            "two" => num.push('2'),
            "three" => num.push('3'),
            "four" => num.push('4'),
            "five" => num.push('5'),
            "six" => num.push('6'),
            "seven" => num.push('7'),
            "eight" => num.push('8'),
            "nine" => num.push('9'),
            _ => panic!("This shouldn't happen"),
        }

        let num = num.parse::<usize>().unwrap();
        sum_part_02 += num;
    }

    println!("Part 01 Sum: {}", sum_part_02);
}
