use std::{
    cmp::{max, min},
    collections::HashMap,
    hash::Hash,
};

use advent_25::input::{get_input_lines, get_input_lines_ex};

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
    fn intersects(&self, point: &Point) -> bool {
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
}

struct Edges {
    /// edges where the 0 coord is the same
    edges: HashMap<i64, (i64, i64)>,
}

impl Edges {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    fn push(&mut self, edge: &Edge) {
        if edge.p1.0 == edge.p2.0 {
            let low = min(edge.p1.1, edge.p2.1);
            let high = max(edge.p1.1, edge.p2.1);
            self.edges.insert(edge.p1.0, (low, high));
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
        let upper = min(self.c1.0, self.c2.0);
        let left = min(self.c1.1, self.c2.1);
        let lower = max(self.c1.0, self.c2.0);
        let right = max(self.c1.1, self.c2.1);

        (upper..=lower)
            .flat_map(|row| (left..=right).map(move |col| (row, col)))
            .collect()
    }

    fn test_within(&self, edges: &Edges) -> bool {
        for point in self.points() {
            let crossings = edges.count_crossings(&point);
            if crossings % 2 == 0 {
                println!("Rejecting {:?}", point);
                // Outside a polygon or not on an edge
                return false;
            }
        }
        // all points in a polygon or on an edge
        true
    }

    fn interior_edges(&self, edges: &[Edge]) -> bool {
        edges
            .iter()
            .any(|e| self.point_interior(&e.p1) || self.point_interior(&e.p2) || (self.point_on_border_not_corner(&e.p1) || self.point_on_border_not_corner(&e.p2)))
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

    dbg!(&edges_raw);

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
    for rectangle in possible_rectangles {
        println!("Testing {rectangle:?}");
        // if rectangle.test_within(&edges) {
        //     println!("Largest fully covered rectangle: {rectangle:?}");
        //     break;
        // }

        if !rectangle.interior_edges(&edges_raw) {
            println!("Largest fully covered rectangle: {rectangle:?}");
            break;
        }
    }
}
