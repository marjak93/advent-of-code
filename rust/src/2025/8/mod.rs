use crate::util::get_input;
use std::collections::HashMap;

type JunctionBox = (f64, f64, f64);
type Circuit = Vec<usize>;

fn calculate_distance(a: &JunctionBox, b: &JunctionBox) -> f64 {
    let (x1, y1, z1) = a;
    let (x2, y2, z2) = b;

    ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt()
}

fn create_circuits(
    junction_boxes: &HashMap<usize, JunctionBox>,
    max_iterations: usize,
) -> (Vec<Circuit>, Option<(usize, usize)>) {
    let mut last: Option<(usize, usize)> = None;
    let mut distances: Vec<(usize, usize, f64)> = Vec::new();

    // Calculate all pairwise distances
    for i in 0..junction_boxes.len() {
        for j in (i + 1)..junction_boxes.len() {
            let dist = calculate_distance(&junction_boxes[&i], &junction_boxes[&j]);
            distances.push((i, j, dist));
        }
    }

    // Sort by distance (ascending)
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    // Initialize circuits - each junction box starts in its own circuit
    let mut circuits: Vec<Circuit> = (0..junction_boxes.len()).map(|i| vec![i]).collect();

    let mut iteration = 0;

    while iteration < max_iterations && !distances.is_empty() {
        let (a, b, _) = distances.remove(0);

        // Find which circuits contain a and b
        let circuit_a_index = circuits.iter().position(|c| c.contains(&a));
        let circuit_b_index = circuits.iter().position(|c| c.contains(&b));

        if let (Some(ca_idx), Some(cb_idx)) = (circuit_a_index, circuit_b_index) {
            // Already connected
            if ca_idx == cb_idx {
                iteration += 1;
                continue;
            }

            // Merge circuits
            let circuit_b = circuits.remove(cb_idx);
            circuits[if ca_idx > cb_idx { ca_idx - 1 } else { ca_idx }].extend(circuit_b);

            // Stop if only one circuit remains
            if circuits.len() == 1 {
                last = Some((a, b));
                break;
            }
        }

        iteration += 1;
    }

    (circuits, last)
}

pub fn part1() {
    let input = get_input(2025, 8);

    let junction_boxes: HashMap<usize, JunctionBox> = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(i, line)| {
            let coords: Vec<f64> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            (i, (coords[0], coords[1], coords[2]))
        })
        .collect();

    let (circuits, _) = create_circuits(&junction_boxes, 1000);

    let mut lengths: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
    lengths.sort_by(|a, b| b.cmp(a));

    let three_largest: Vec<usize> = lengths.iter().take(3).copied().collect();
    let result: usize = three_largest.iter().product();

    println!("{}", result);
}

pub fn part2() {
    let input = get_input(2025, 8);

    let junction_boxes: HashMap<usize, JunctionBox> = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(i, line)| {
            let coords: Vec<f64> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            (i, (coords[0], coords[1], coords[2]))
        })
        .collect();

    let (_, last) = create_circuits(&junction_boxes, usize::MAX);

    if let Some((a, b)) = last {
        let box_a = junction_boxes.get(&a).unwrap();
        let box_b = junction_boxes.get(&b).unwrap();

        let result = (box_a.0 as i64) * (box_b.0 as i64);
        println!("{}", result);
    }
}
