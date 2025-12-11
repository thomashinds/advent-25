use std::{cmp::{max, min}, collections::{HashMap, HashSet}};

use advent_25::input::{get_input_lines, get_input_lines_ex};

fn main() {
    let points: Vec<_> = get_input_lines(8).iter().map(parse_point).collect();

    let mut distances = HashMap::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dist = distance(points[i], points[j]);
            distances.insert((i, j), dist);
        }
    }

    let mut sorted_distances: Vec<_> = distances.iter().collect();
    sorted_distances.sort_by(|(_pa, dist_a), (_pb, dist_b)| dist_a.total_cmp(dist_b));
    // sorted_distances.reverse();

    let mut circuits: Vec<Vec<usize>> = Vec::new();
    let mut direct_connections: HashSet<(usize, usize)> = HashSet::new();
    for _ in 0..1000 {
        let &(&(p1, p2), dist) = sorted_distances
            .iter()
            .find(|((p1, p2), _dist)| !direct_connections.contains(&(*p1, *p2)))
            .unwrap();

        // println!(
        //     "Shortest path is between {:?} and {:?} (dist {}",
        //     points[p1], points[p2], dist
        // );

        assert!(p1 < p2);
        direct_connections.insert((p1, p2));

        let circuit_index_p1 = circuits
            .iter()
            .enumerate()
            .find(|(_n, circuit)| circuit.contains(&(p1)))
            .map(|(n, _circuit)| n);

        let circuit_index_p2 = circuits
            .iter()
            .enumerate()
            .find(|(_n, circuit)| circuit.contains(&(p2)))
            .map(|(n, _circuit)| n);

        match (circuit_index_p1, circuit_index_p2) {
            (None, None) => circuits.push(vec![p1, p2]),
            (Some(circuit_index_p1), None) => circuits[circuit_index_p1].push(p2),
            (None, Some(circuit_index_p2)) => circuits[circuit_index_p2].push(p1),
            (Some(circuit_index_p1), Some(circuit_index_p2)) => {
                if circuit_index_p1 == circuit_index_p2
                {

                    continue;
                }
                // println!("Joining circuits {:?} and {:?}", circuits[circuit_index_p1], circuits[circuit_index_p2]);
                let to_remove = max(circuit_index_p1, circuit_index_p2);
                let to_add_to = min(circuit_index_p1, circuit_index_p2);
                let mut circuit = circuits.remove(to_remove);
                circuits[to_add_to].append(&mut circuit);
            }
        }
    }

    circuits.sort_by_key(Vec::len);
    let product_of_longest_3 = circuits.iter().rev().take(3).map(Vec::len).product::<usize>();
    println!("Product of longest 3: {product_of_longest_3}");

    for _ in 0.. {
        let &(&(p1, p2), dist) = sorted_distances
            .iter()
            .find(|((p1, p2), _dist)| !direct_connections.contains(&(*p1, *p2)))
            .unwrap();

        // println!(
        //     "Shortest path is between {:?} and {:?} (dist {})",
        //     points[p1], points[p2], dist
        // );

        assert!(p1 < p2);
        direct_connections.insert((p1, p2));

        let circuit_index_p1 = circuits
            .iter()
            .enumerate()
            .find(|(_n, circuit)| circuit.contains(&(p1)))
            .map(|(n, _circuit)| n);

        let circuit_index_p2 = circuits
            .iter()
            .enumerate()
            .find(|(_n, circuit)| circuit.contains(&(p2)))
            .map(|(n, _circuit)| n);

        match (circuit_index_p1, circuit_index_p2) {
            (None, None) => circuits.push(vec![p1, p2]),
            (Some(circuit_index_p1), None) => circuits[circuit_index_p1].push(p2),
            (None, Some(circuit_index_p2)) => circuits[circuit_index_p2].push(p1),
            (Some(circuit_index_p1), Some(circuit_index_p2)) => {
                if circuit_index_p1 == circuit_index_p2
                {

                    continue;
                }
                // println!("Joining circuits {:?} and {:?}", circuits[circuit_index_p1], circuits[circuit_index_p2]);
                let to_remove = max(circuit_index_p1, circuit_index_p2);
                let to_add_to = min(circuit_index_p1, circuit_index_p2);
                let mut circuit = circuits.remove(to_remove);
                circuits[to_add_to].append(&mut circuit);
                
            }
        }
        
        if circuits.len() == 1 && circuits[0].len() == points.len() {
        println!(
            "Connected {:?} and {:?} (X*X = {})",
            points[p1], points[p2], points[p1].0 as i64 * points[p2].0 as i64
        );
        break;
        }
    }
}

fn parse_point(s: impl AsRef<str>) -> (i32, i32, i32) {
    let mut parts = s.as_ref().split(',').map(|p| p.parse().unwrap());
    (
        parts.next().unwrap(),
        parts.next().unwrap(),
        parts.next().unwrap(),
    )
}

fn distance(p1: (i32, i32, i32), p2: (i32, i32, i32)) -> f64 {
    f64::from(((p1.0 - p2.0) as f32).powf(2.0) + ((p1.1 - p2.1) as f32).powf(2.0) + ((p1.2 - p2.2) as f32).powf(2.0)).sqrt()
}
