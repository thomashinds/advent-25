use advent_25::input::get_input_ex;
use std::fmt::Debug;

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

    fn h_flip(&self) -> Self {
        let mut shape = [[false; 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                shape[row][col] = self.shape[row][2 - col];
            }
        }
        Self { shape }
    }

    fn v_flip(&self) -> Self {
        let mut shape = [[false; 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                shape[row][col] = self.shape[2 - row][col];
            }
        }
        Self { shape }
    }

    fn d_flip(&self) -> Self {
        let mut shape = [[false; 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                shape[row][col] = self.shape[col][row];
            }
        }
        Self { shape }
    }
}

impl Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.shape {
            for cell in line {
                if cell {
                    write!(f, "#").unwrap();
                } else {
                    write!(f, ".").unwrap();
                }
            }
            writeln!(f).unwrap();
        }
        writeln!(f)
    }
}

#[derive(Debug)]
struct TransformedShape {
    versions: [Shape; 8],
}

impl TransformedShape {
    fn new(s: &str) -> Self {
        let shape = Shape::new(s);

        let versions = [
            shape.clone(),
            shape.clone().h_flip(),
            shape.clone().v_flip(),
            shape.clone().h_flip().v_flip(),
            shape.clone().d_flip(),
            shape.clone().h_flip().d_flip(),
            shape.clone().v_flip().d_flip(),
            shape.h_flip().v_flip().d_flip(),
        ];
        Self { versions }
    }
}

#[derive(Debug)]
struct Region {
    rows: i32,
    cols: i32,
    required_shapes: Vec<i32>
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
            rows, cols, required_shapes
        }
    }

    fn fit(shapes: &[TransformedShape], cells: Vec<Vec<bool>>) -> bool {
        for row in self.rows
    }
}

fn main() {
    let input = get_input_ex(12);
    let (shape_input, region_input) = input.rsplit_once("\n\n").unwrap();
    let shapes: Vec<_> = shape_input.split("\n\n").map(TransformedShape::new).collect();
    
    let regions: Vec<_> = region_input.lines().map(Region::new).collect();
    dbg!(&regions);
}
