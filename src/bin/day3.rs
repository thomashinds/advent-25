use std::ops::Index;

use advent_25::input::get_input_lines;

fn main() {
    let banks = get_input_lines(3);
    let total_jolt: u64 = banks.iter().map(|b| max_jolt(b)).sum();
    println!("Total joltage 2: {total_jolt}");

    let banks: Vec<Vec<u64>> = banks
        .iter()
        .map(|bank| {
            bank.chars()
                .map(|c| u64::from(c.to_digit(10).unwrap()))
                .collect()
        })
        .collect();

    // for bank in banks {
    //     println!("First bank {:?}", bank);
    //     println!("Jolt12 of first bank: {}", max_joltn(&bank, 12));
    // }
    let total_jolt12: u64 = banks.iter().map(|b| max_joltn(b, 12)).sum();
    println!("Total jolt12: {total_jolt12}");
}

fn max_jolt(bank: &str) -> u64 {
    (11..=99)
        .rev()
        .find(|&jolt| valid_jolt(bank, jolt))
        .unwrap()
}

fn valid_jolt(bank: &str, jolt: u64) -> bool {
    let jolt = jolt.to_string();
    let mut jolt = jolt.chars();
    let first = jolt.next().unwrap();
    let second = jolt.next().unwrap();

    if let Some((_, remaining)) = bank.split_once(first) {
        remaining.contains(second)
    } else {
        false
    }
}

fn max_joltn(bank: &[u64], digits: u32) -> u64 {
    if digits == 0 {
        return 0;
    }
    for d in (1..=9).rev() {
        // Make sure there are enough digits left after the split
        if let Some(index) = bank
            .iter()
            .enumerate()
            .find(|(_i, n)| **n == d)
            .map(|(i, _d)| i)
        {
            // println!("Found a {d} at index {index}");
            if digits <= (bank.len() - index) as u32 {
                return d * 10u64.pow(digits - 1) + max_joltn(&bank[index + 1..], digits - 1);
            }
        }
    }

    todo!()
}
