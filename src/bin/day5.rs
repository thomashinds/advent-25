use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

use advent_25::input::{get_input, get_input_ex};

fn main() {
    let input = get_input(5);
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges = ranges.split('\n').map(parse_range);
    let ids = ids
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.parse::<i64>().unwrap());

    let num_fresh = ids
        .filter(|id| ranges.clone().any(|r| r.contains(id)))
        .count();
    println!("{num_fresh} fresh ids");

    let ranges: Vec<_> = ranges.collect();
    let mut unique_ranges = RangeUnique::default();
    for range in ranges {
        unique_ranges.add(range);
    }

    println!("{} total possible fresh ids", unique_ranges.total());
}

fn parse_range(range: &str) -> RangeInclusive<i64> {
    let (begin, end) = range.trim().split_once('-').unwrap();
    let begin: i64 = begin.parse().unwrap();
    let end: i64 = end.parse().unwrap();

    begin..=end
}

#[derive(Default)]
struct RangeUnique {
    internal_ranges: Vec<RangeInclusive<i64>>,
}

impl RangeUnique {
    fn add(&mut self, mut range: RangeInclusive<i64>) {
        loop {
            let mut coalesced = false;
            for i in (0..self.internal_ranges.len()).rev() {
                if let Some(new_range) = coalesce(&range, &self.internal_ranges[i]) {
                    self.internal_ranges.remove(i);
                    range = new_range;
                    coalesced = true;
                }
            }
            if !coalesced {
                break;
            }
        }

        self.internal_ranges.push(range);

        self.validate();
    }

    fn validate(&self) {
        for i in 0..self.internal_ranges.len() - 1 {
            for j in i + 1..self.internal_ranges.len() {
                if coalesce(&self.internal_ranges[i], &self.internal_ranges[j]).is_some() {
                    println!("Error! Overlap detected:");
                    println!("Range 1: {:?}", self.internal_ranges[i]);
                    println!("Range 2: {:?}", self.internal_ranges[j]);
                    panic!();
                }
            }
        }
    }

    fn total(&self) -> i64 {
        self.validate();
        self.internal_ranges
            .iter()
            .map(|r| 1 + r.end() - r.start())
            .sum()
    }
}

fn coalesce(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> Option<RangeInclusive<i64>> {
    if r2.contains(r1.start())
        || r2.contains(r1.end())
        || r1.contains(r2.start())
        || r1.contains(r2.end())
    {
        let start = *min(r1.start(), r2.start());
        let end = *max(r1.end(), r2.end());
        Some(RangeInclusive::new(start, end))
    } else {
        None
    }
}
