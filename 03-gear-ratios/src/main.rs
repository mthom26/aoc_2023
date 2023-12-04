use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut symbols = Vec::new();
    symbols.push(Vec::new());
    let mut numbers = Vec::new();

    let mut temp_numbers = Vec::new();

    let mut sum = 0;

    for line in input.lines() {
        let mut new_symbols = Vec::new();
        let mut editing_num = false;
        let mut new_numbers = Vec::new();
        let mut new_num = String::new();

        for (i, c) in line.char_indices() {
            if c.is_numeric() {
                editing_num = true;
                new_num.push(c);
            } else if c == '.' {
                if editing_num {
                    new_numbers.push((i - new_num.len(), new_num.clone()));
                    editing_num = false;
                    new_num.clear();
                }
            } else {
                if editing_num {
                    new_numbers.push((i - new_num.len(), new_num.clone()));
                    editing_num = false;
                    new_num.clear();
                }
                new_symbols.push(i);
            }
        }
        // If there is a number as the last character on a row the previous for loop
        // stops before being able to add the new number so add it here if it exists
        if !new_num.is_empty() {
            new_numbers.push((140 - new_num.len(), new_num));
        }
        symbols.push(new_symbols);
        numbers.push(new_numbers);
    }
    symbols.push(Vec::new());

    for (row_index, row) in numbers.iter().enumerate() {
        let mut temp_nums = Vec::new();
        // if row_index < 79 {
        //     continue;
        // }
        // if row_index == 80 {
        //     break;
        // }
        'current_number: for (num_index, num) in row.iter() {
            let start = if *num_index == 0 { 0 } else { num_index - 1 };
            let end = start + num.len() + 1;

            for i in symbols[row_index].iter() {
                if *i >= start && *i <= end {
                    // Number is adjacent to symbol
                    let n = num.parse::<usize>().unwrap();
                    temp_nums.push(n);
                    sum += n;
                    continue 'current_number;
                }
            }
            for i in symbols[row_index + 1].iter() {
                if *i >= start && *i <= end {
                    // Number is adjacent to symbol
                    let n = num.parse::<usize>().unwrap();
                    temp_nums.push(n);
                    sum += n;
                    continue 'current_number;
                }
            }
            for i in symbols[row_index + 2].iter() {
                if *i >= start && *i <= end {
                    // Number is adjacent to symbol
                    let n = num.parse::<usize>().unwrap();
                    temp_nums.push(n);
                    sum += n;
                    continue 'current_number;
                }
            }
        }
        temp_numbers.push(temp_nums);
    }

    println!("Part 01 sum {:?}", sum);
    // println!("Temp numbers");
    // for (i, n) in temp_numbers.iter().enumerate() {
    //     println!("{}: {:?}", i + 1, n);
    // }
}
