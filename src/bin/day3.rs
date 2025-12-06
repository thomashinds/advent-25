use advent_25::input::get_input_lines;

fn main() {
    let banks = get_input_lines(3);
    let total_jolt: i32 = banks.iter().map(|b| max_jolt(b)).sum();

    println!("Total joltage 2: {total_jolt}");
}

fn max_jolt(bank: &str) -> i32
{
    (11..=99).rev().find(|&jolt| valid_jolt(bank, jolt)).unwrap()
}

fn valid_jolt(bank: &str, jolt: i32) -> bool
{
    let jolt = jolt.to_string();
    let mut jolt = jolt.chars();
    let first = jolt.next().unwrap();
    let second = jolt.next().unwrap();

    if let Some((_, remaining)) = bank.split_once(first)
    {
        remaining.contains(second)
    }
    else {
        false
    }
}

