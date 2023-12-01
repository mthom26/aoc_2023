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
}
