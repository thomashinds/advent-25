use std::{
    cmp::{max, min},
    collections::HashMap,
    hash::Hash,
};

use advent_25::input::{get_input_lines, get_input_lines_ex};

use rayon::prelude::*;

type Point = (i64, i64);

fn main() {
    let points: Vec<Point> = get_input_lines(9)
        .iter()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    part1(&points);

    part2(&points);
}

fn part1(points: &[Point]) {
    let mut max_area = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let area = ((1 + points[i].0 - points[j].0) * (1 + points[i].1 - points[j].1)).abs();
            max_area = max(area, max_area);
        }
    }

    println!("Max area: {max_area}");
}

#[derive(Clone, Debug)]
struct Edge {
    p1: Point,
    p2: Point,
}

impl Edge {
    fn contains(&self, point: &Point) -> bool {
        // Either matches x or y
        if self.p1.0 == self.p2.0 {
            if point.0 == self.p1.0 {
                let low = min(self.p1.1, self.p2.1);
                let high = max(self.p1.1, self.p2.1);
                return point.1 >= low && point.1 <= high;
            }
        } else if self.p1.1 == self.p2.1 {
            if point.1 == self.p1.1 {
                let low = min(self.p1.0, self.p2.0);
                let high = max(self.p1.0, self.p2.0);
                return point.0 >= low && point.0 <= high;
            }
        } else {
            panic!("bad edge!");
        }
        false
    }

    fn same_x(&self) -> bool {
        if self.p1.0 == self.p2.0 {
            true
        } else if self.p1.1 == self.p2.1 {
            false
        } else {
            panic!()
        }
    }

    fn length(&self) -> i64 {
        max((self.p1.1 - self.p2.1).abs(), (self.p1.0 - self.p2.0).abs())
    }

    fn intersects(&self, other: &Self) -> bool {
        // Overlapping edges do not count as intersecting, and parallel edges cannot intersect
        if self.same_x() == other.same_x() {
            return false;
        }

        let intersection_point = if self.same_x() {
            (self.p1.0, other.p1.1)
        } else {
            (other.p1.0, self.p1.1)
        };

        if self.contains(&intersection_point) && other.contains(&intersection_point) {
            println!("  {self:?} intersects {other:?} at {intersection_point:?}");
            return true;
        }
        false
    }
}

struct Edges {
    /// edges where the 0 coord is the same
    edges: HashMap<i64, (i64, i64)>,

    all_edges_raw: Vec<Edge>
}

impl Edges {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
            all_edges_raw: Vec::new()
        }
    }

    fn push(&mut self, edge: &Edge) {
        self.all_edges_raw.push(edge.clone());
        if edge.p1.0 == edge.p2.0 {
            let low = min(edge.p1.1, edge.p2.1);
            let high = max(edge.p1.1, edge.p2.1);
            let existing = self.edges.insert(edge.p1.0, (low+1, high));
            assert!(existing.is_none());
        } else if edge.p1.1 == edge.p2.1 {
            // who cares
        } else {
            panic!("bad edge!");
        }
    }

    fn count_crossings(&self, point: &Point) -> i64 {
        self.edges
            .iter()
            .filter(|&(&row, &(col_low, col_high))| {
                row <= point.0 && col_low <= point.1 && col_high >= point.1
            })
            .count() as i64
    }
}

#[derive(Debug)]
struct Rectangle {
    c1: Point,
    c2: Point,
    area: i64,
}

impl Rectangle {
    const fn new(c1: Point, c2: Point) -> Self {
        let area = ((1 + (c1.0 - c2.0).abs()) * (1 + (c1.1 - c2.1).abs()));
        Self { c1, c2, area }
    }

    fn points(&self) -> Vec<Point> {
        // list of all points along edges
        let upper = min(self.c1.0, self.c2.0);
        let left = min(self.c1.1, self.c2.1);
        let lower = max(self.c1.0, self.c2.0);
        let right = max(self.c1.1, self.c2.1);

        let top = (left..=right).map(|y| (upper, y));
        let bottom = (left..=right).map(|y| (lower, y));
        let leftside = (upper..=lower).map(|x| (x, left));
        let rightside = (upper..=lower).map(|x| (x, right));

        top.chain(bottom).chain(leftside).chain(rightside).collect()

        // let upper = min(self.c1.0, self.c2.0);
        // let left = min(self.c1.1, self.c2.1);
        // let lower = max(self.c1.0, self.c2.0);
        // let right = max(self.c1.1, self.c2.1);

        // (upper..=lower)
        //     .flat_map(|row| (left..=right).map(move |col| (row, col)))
        //     .collect()
    }

    fn edges(&self) -> [Edge; 4] {
        let upper = min(self.c1.0, self.c2.0);
        let left = min(self.c1.1, self.c2.1);
        let lower = max(self.c1.0, self.c2.0);
        let right = max(self.c1.1, self.c2.1);

        [
            Edge {
                p1: (upper, left),
                p2: (upper, right),
            },
            Edge {
                p1: (upper, right),
                p2: (lower, right),
            },
            Edge {
                p1: (lower, right),
                p2: (lower, left),
            },
            Edge {
                p1: (lower, left),
                p2: (upper, left),
            },
        ]
    }

    fn test_within(&self, edges: &Edges) -> bool {
        for point in self.points() {
            // first check if it's on an edge
            if edges.all_edges_raw.iter().any(|e| e.contains(&point)) {
                continue;
            }
            // then if it's not on an edge, test interiorness
            let crossings = edges.count_crossings(&point);
            if crossings % 2 == 0 {
                // println!("Rejecting {point:?} with {crossings} crossings");
                // Outside a polygon or not on an edge
                return false;
            }
        }
        // all points in a polygon or on an edge
        true
    }

    fn interior_edges(&self, edges: &[Edge]) -> bool {
        edges.iter().any(|e| {
            self.point_interior(&e.p1)
                || self.point_interior(&e.p2)
                || (self.point_on_border_not_corner(&e.p1)
                    || self.point_on_border_not_corner(&e.p2))
        })
    }

    fn edges_intersect(&self, edges: &[Edge]) -> bool {
        edges
            .iter()
            .any(|e| self.edges().iter().any(|se| se.intersects(e)))
    }

    fn point_interior(&self, point: &Point) -> bool {
        let min_0 = min(self.c1.0, self.c2.0);
        let min_1 = min(self.c1.1, self.c2.1);
        let max_0 = max(self.c1.0, self.c2.0);
        let max_1 = max(self.c1.1, self.c2.1);

        point.0 > min_0 && point.0 < max_0 && point.1 > min_1 && point.1 < max_1
    }

    fn point_on_border_not_corner(&self, point: &Point) -> bool {
        let min_0 = min(self.c1.0, self.c2.0);
        let min_1 = min(self.c1.1, self.c2.1);
        let max_0 = max(self.c1.0, self.c2.0);
        let max_1 = max(self.c1.1, self.c2.1);

        if point.0 == min_0 || point.0 == max_0 {
            return point.1 > min_1 && point.1 < max_1;
        }
        if point.1 == min_1 || point.1 == max_1 {
            return point.0 > min_0 && point.0 < max_0;
        }
        false
    }
}

fn part2(points: &[Point]) {
    let mut edges_raw: Vec<Edge> = points
        .windows(2)
        .map(|w| Edge { p1: w[0], p2: w[1] })
        .collect();

    edges_raw.push(Edge {
        p1: points[points.len() - 1],
        p2: points[0],
    });

    let mut edges = Edges::new();

    for edge in edges_raw.clone() {
        edges.push(&edge);
    }

    edges.all_edges_raw.sort_by_key(|e| e.length());
    edges.all_edges_raw.reverse();

    // dbg!(&edges_raw);

    let mut possible_rectangles = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            possible_rectangles.push(Rectangle::new(points[i], points[j]));
        }
    }

    possible_rectangles.sort_by_key(|r| r.area);
    possible_rectangles.reverse();

    println!("Largest rectangle: {:?}", possible_rectangles[0]);
    println!("{} total rectangles to check", possible_rectangles.len());
    println!();
    let largest_fully_covered_rectangle = possible_rectangles.par_iter().find_first(|rectangle| rectangle.test_within(&edges));
        // println!("Testing {rectangle:?}");
        // if (i%100) == 0 {
        //     println!("Testing #{i}");
        // }

        

        // if !rectangle.interior_edges(&edges_raw) {
        //     println!("Largest fully covered rectangle: {rectangle:?}");
        //     break;
        // }

        // if !rectangle.edges_intersect(&edges_raw) {
        println!("Largest fully covered rectangle: {largest_fully_covered_rectangle:?}");
        //     break;
        // }
    
}
