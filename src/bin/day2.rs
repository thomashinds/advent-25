use std::{collections::BTreeSet, ops::RangeInclusive};

use advent_25::input::get_input;

fn main() {
    let ranges: Vec<_> = get_input(2).split(',').map(parse_range).collect();
    let max = ranges
        .iter()
        .map(|range| range.clone().max().unwrap())
        .max()
        .unwrap();

    let max_digits = format!("{max}").len();

    let mut sum_invalid = 0;
    for i in 0.. {
        let n = repeated(i, 2);
        if n > max {
            break;
        }
        if ranges.iter().any(|r| r.contains(&n)) {
            sum_invalid += n;
        }
    }

    println!("Sum of all invalid ids based on 2x in ranges: {sum_invalid}");

    let mut invalid_ids = BTreeSet::new();
    for i in 0.. {
        if repeated(i, 2) > max {
            break;
        }
        for k in 2..=max_digits {
            let n = repeated(i, k);
            if n > max {
                break;
            }
            if ranges.iter().any(|r| r.contains(&n)) {
                invalid_ids.insert(n);
            }
        }
    }

    let sum_invalid_ids: i64 = invalid_ids.iter().sum();
    println!("Sum of all invalid ids based on Nx in ranges: {sum_invalid_ids}");
}

fn parse_range(range: &str) -> RangeInclusive<i64> {
    let (begin, end) = range.trim().split_once('-').unwrap();
    // println!("parse: [{begin}]");
    let begin: i64 = begin.parse().unwrap();
    // println!("parse: [{end}]");
    let end: i64 = end.parse().unwrap();

    begin..=end
}

fn repeated(n: i64, k: usize) -> i64 {
    format!("{n}").repeat(k).parse().unwrap()
}
