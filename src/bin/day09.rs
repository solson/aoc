extern crate aoc;
extern crate permutohedron;

use std::{cmp, i32};
use std::collections::HashMap;

fn main() {
    let input = aoc::read_file("input/day09");

    let mut city_indices = HashMap::new();
    let mut num_cities = 0;

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();

        for &city in &[words[0], words[2]] {
            city_indices.entry(city).or_insert_with(|| {
                let i = num_cities;
                num_cities += 1;
                i
            });
        }
    }

    let mut distances = vec![0; num_cities * num_cities];

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let source = city_indices[words[0]];
        let target = city_indices[words[2]];
        let distance = words[4].parse::<i32>().unwrap();

        distances[source * num_cities + target] = distance;
        distances[target * num_cities + source] = distance;
    }

    let mut indices: Vec<usize> = (0..num_cities).collect();
    let path_permutations = permutohedron::Heap::new(&mut indices[..]);

    let path_distances = path_permutations.map(|path| {
        let mut total_distance = 0;

        for pair in path.windows(2) {
            let source = pair[0];
            let target = pair[1];

            total_distance += distances[source * num_cities + target];
        }

        total_distance
    });

    let (min, max) = path_distances.fold((i32::MAX, i32::MIN), |(min, max), dist| {
        (cmp::min(dist, min), cmp::max(dist, max))
    });

    println!("The shortest path has length {}.", min);
    println!("The longest path has length {}.", max);
}
