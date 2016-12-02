extern crate aoc;

use std::collections::HashMap;

fn main() {
    let input = aoc::read_file("input/day05");
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let mut nice_count = 0;

    'word: for word in input.lines() {
        let mut vowels = 0;
        let mut last_letter = '\0';
        let mut has_double_letter = false;

        for c in word.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
                _ => {}
            }

            // Naughty letter pairs: ab, cd, pq, or xy
            match (last_letter, c) {
                ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => continue 'word,
                _ => {}
            }

            if !has_double_letter && c == last_letter {
                has_double_letter = true;
            }

            last_letter = c;
        }

        if vowels >= 3 && has_double_letter {
            nice_count += 1;
        }
    }

    println!("There are {} nice words using ruleset 1.", nice_count);
}

fn part_2(input: &str) {
    let mut nice_count = 0;

    for word in input.lines() {
        let word = word.as_bytes();
        let mut adjacency_counts = HashMap::new();
        let mut previous_adjacent = false;
        let mut has_gap_pair = false;

        for window in word.windows(3) {
            let c1 = window[0];
            let c2 = window[1];
            let c3 = window[2];

            if !previous_adjacent || c1 != c2 {
                let count = adjacency_counts.entry((c1, c2)).or_insert(0);
                *count += 1;
                previous_adjacent = c1 == c2;
            } else {
                previous_adjacent = false;
            }

            if !has_gap_pair && c1 == c3 {
                has_gap_pair = true;
            }
        }

        // Check for one final adjacency at the end.
        let c1 = word[word.len() - 2];
        let c2 = word[word.len() - 1];
        if !previous_adjacent || c1 != c2 {
            let count = adjacency_counts.entry((c1, c2)).or_insert(0);
            *count += 1;
        }

        let has_double_pair = adjacency_counts.values().any(|&x| x >= 2);
        if has_gap_pair && has_double_pair {
            nice_count += 1;
        }
    }

    println!("There are {} nice words using ruleset 2.", nice_count);
}
