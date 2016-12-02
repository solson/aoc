extern crate aoc;

use std::cmp::{max, min};

/// Position labelling:
///
///  |0 1 2
/// -+-----
/// 0|1 2 3
/// 1|4 5 6
/// 2|7 8 9
///
/// Number "5" has position (1, 1).
fn position_to_number(position: (i64, i64)) -> i64 {
    match position {
        (0, 0) => 1,
        (1, 0) => 2,
        (2, 0) => 3,
        (0, 1) => 4,
        (1, 1) => 5,
        (2, 1) => 6,
        (0, 2) => 7,
        (1, 2) => 8,
        (2, 2) => 9,
        _ => panic!(),
    }
}

fn main() {
    let input = aoc::read_file("input/day02");
    let mut pos: (i64, i64) = (1, 1); // Start on "5".

    for line in input.trim().split("\n") {
        for direction in line.chars() {
            match direction {
                'U' => pos.1 = max(0, pos.1 - 1),
                'D' => pos.1 = min(2, pos.1 + 1),
                'L' => pos.0 = max(0, pos.0 - 1),
                'R' => pos.0 = min(2, pos.0 + 1),
                _ => panic!(),
            }
        }

        print!("{}", position_to_number(pos));
    }

    println!("");
}
