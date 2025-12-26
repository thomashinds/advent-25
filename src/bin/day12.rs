use std::collections::HashMap;

use advent_25::input::{get_input, get_input_ex};

fn main() {
    let input = get_input(12);
    let (shape_input, region_input) = input.rsplit_once("\n\n").unwrap();
    let shapes: Vec<_> = shape_input.split("\n\n").map(Shape::new).collect();

    let regions: Vec<_> = region_input.lines().map(Region::new).collect();

    let mut counts: HashMap<FitResult, i32> = HashMap::new();
    for result in regions.iter().map(|r| would_fit(r, &shapes)) {
        *counts.entry(result).or_insert(0) += 1;
    }
    dbg!(&counts);
}

#[derive(Clone)]
struct Shape {
    shape: [[bool; 3]; 3],
}

#[allow(clippy::needless_range_loop)]
impl Shape {
    fn new(s: &str) -> Self {
        let mut shape = [[false; 3]; 3];
        for (row, line) in s.split('\n').skip(1).enumerate() {
            for (col, c) in line.chars().enumerate() {
                shape[row][col] = match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!(),
                };
            }
        }
        Self { shape }
    }

    fn count_filled(&self) -> i32 {
        self.shape.as_flattened().iter().filter(|&&e| e).count() as i32
    }
}

#[derive(Debug)]
struct Region {
    rows: i32,
    cols: i32,
    required_shapes: Vec<i32>,
}

impl Region {
    fn new(s: &str) -> Self {
        let (dims, reqs) = s.split_once(": ").unwrap();
        let cols = dims.split_once('x').unwrap().0.parse().unwrap();
        let rows = dims.split_once('x').unwrap().1.parse().unwrap();
        let mut required_shapes = Vec::new();
        for count in reqs.split(' ') {
            required_shapes.push(count.parse().unwrap());
        }
        Self {
            rows,
            cols,
            required_shapes,
        }
    }

    const fn area(&self) -> i32 {
        self.rows * self.cols
    }

    const fn area_3x3(&self) -> i32 {
        9 * (self.rows / 3) * (self.cols / 3)
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum FitResult {
    Fits,
    DoesNotFit,
    MaybeFits,
}

fn would_fit(region: &Region, shapes: &[Shape]) -> FitResult {
    let region_area = region.area();
    let region_3x3_area = region.area_3x3();
    let no_overlap_required_area = region.required_shapes.iter().sum::<i32>() * 9;
    let perfect_tiling_required_area = region
        .required_shapes
        .iter()
        .enumerate()
        .map(|(i, num)| shapes[i].count_filled() * num)
        .sum();

    if region_3x3_area >= no_overlap_required_area {
        FitResult::Fits
    } else if region_area < perfect_tiling_required_area {
        FitResult::DoesNotFit
    } else {
        println!("Region area of {region_area} is between perfect tiling area {perfect_tiling_required_area} and no overlap area {no_overlap_required_area}");
        FitResult::MaybeFits
    }
}
