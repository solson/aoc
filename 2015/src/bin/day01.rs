extern crate aoc;

fn main() {
    let input = aoc::read_file("input/day01");

    let mut floor = 0;
    let mut reported_basement = false;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }

        if !reported_basement && floor == -1 {
            println!("Entered basement at position {}.", i + 1);
            reported_basement = true;
        }
    }

    println!("Stopped at floor {}.", floor);
}
