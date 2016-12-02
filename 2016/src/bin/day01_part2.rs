extern crate aoc;

use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
enum Rotation { Left, Right }

#[derive(Copy, Clone, Debug)]
enum Direction { North, South, East, West }

fn rotate(direction: Direction, rotation: Rotation) -> Direction {
    use self::Direction::*;
    use self::Rotation::*;
    match (direction, rotation) {
        (North, Left)  => West,
        (North, Right) => East,
        (South, Left)  => East,
        (South, Right) => West,
        (East,  Left)  => North,
        (East,  Right) => South,
        (West,  Left)  => South,
        (West,  Right) => North,
    }
}

fn travel(direction: Direction, distance: i64, position: &mut (i64, i64)) {
    use self::Direction::*;
    match direction {
        North => position.1 += distance,
        South => position.1 -= distance,
        East  => position.0 += distance,
        West  => position.0 -= distance,
    }
}

fn main() {
    let input = aoc::read_file("input/day01");
    let mut position: (i64, i64) = (0, 0);
    let mut direction = Direction::North;

    let mut visited_locations = HashSet::new();
    visited_locations.insert(position);

    for instruction in input.trim().split(", ") {
        let rotation = match &instruction[0..1] {
            "L" => Rotation::Left,
            "R" => Rotation::Right,
            _ => panic!(),
        };

        let distance: i64 = instruction[1..].parse().unwrap();
        direction = rotate(direction, rotation);

        for _ in 0..distance {
            travel(direction, 1, &mut position);

            // This is the first location we've visited twice if insert returns false.
            if !visited_locations.insert(position) {
                println!("Distance from origin: {}", position.0.abs() + position.1.abs());
                return;
            }
        }
    }

    println!("Visited no location twice.");
}
