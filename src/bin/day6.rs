use advent_25::input::{get_input_lines, get_input_lines_ex};

fn main() {
    let lines = get_input_lines(6);
    let nums: Vec<Vec<i64>> = lines[0..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let mut total = 0;
    for (i, op) in lines[lines.len() - 1].split_ascii_whitespace().enumerate() {
        // dbg!(&nums.iter().map(|row| row[i]));
        match op {
            "+" => total += nums.iter().map(|row| row[i]).sum::<i64>(),
            "*" => total += nums.iter().map(|row| row[i]).product::<i64>(),
            _ => panic!(),
        }
    }
    println!("Total: {total}");

    let mut fixed_lines: Vec<Vec<char>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    for i in 0..lines[0].len() {
        if lines.iter().all(|line| line.chars().nth(i).unwrap() == ' ') {
            // This col is a separator
            for line in &mut fixed_lines {
                line[i] = '|';
            }
        }
    }
    let fixed_lines: Vec<String> = fixed_lines
        .iter()
        .map(|line: &Vec<char>| line.iter().collect())
        .collect();
    for line in &fixed_lines {
        println!("{line}");
    }

    let mut total = 0;
    for (i, op) in lines[lines.len() - 1].split_ascii_whitespace().enumerate() {
        let numbers: Vec<Vec<char>> = fixed_lines
            .iter()
            .map(|line| line.split('|').nth(i).unwrap().chars().collect())
            .collect::<Vec<_>>();
        let numbers = (0..numbers[0].len())
            .map(|n| {
                numbers
                    .iter()
                    .map(move |line| line[n])
                    .collect::<String>().trim()
                    .parse::<i64>()
                    .unwrap()
            });
            
        // dbg!(&numbers.clone().collect::<Vec<_>>());
        // return;
        match op {
            "+" => total += numbers.sum::<i64>(),
            "*" => total += numbers.product::<i64>(),
            _ => panic!(),
        }
    }
    println!("Total: {total}");
}
