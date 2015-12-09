extern crate aoc;

use std::collections::HashMap;

fn main() {
    let input = aoc::read_file("input/day03");
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let mut counts = HashMap::new();
    let mut pos = (0, 0);

    counts.insert(pos, 1);

    for direction in input.chars() {
        let change = match direction {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, 1),
            'v' => (0, -1),
            _ => continue,
        };

        pos.0 += change.0;
        pos.1 += change.1;

        let count = counts.entry(pos).or_insert(0);
        *count += 1;
    }

    println!("{} houses got at least one present.", counts.len());
}

fn part_2(input: &str) {
    let mut counts = HashMap::new();
    let mut santa_pos = (0, 0);
    let mut robo_santa_pos = (0, 0);
    let mut robo_santas_turn = false;

    counts.insert((0, 0), 2);

    for direction in input.chars() {
        let change = match direction {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, 1),
            'v' => (0, -1),
            _ => continue,
        };

        let pos = if robo_santas_turn {
            &mut robo_santa_pos
        } else {
            &mut santa_pos
        };

        robo_santas_turn = !robo_santas_turn;
        pos.0 += change.0;
        pos.1 += change.1;

        let count = counts.entry(*pos).or_insert(0);
        *count += 1;
    }

    println!("{} houses got at least one present.", counts.len());
}
