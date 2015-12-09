extern crate aoc;

fn main() {
    let input = aoc::read_file("input/day02");

    let mut total_paper = 0;
    let mut total_ribbon = 0;

    for line in input.lines() {
        let mut dimensions: Vec<i32> = line.split('x').map(|s| s.parse().unwrap()).collect();
        dimensions.sort();
        let d1 = dimensions[0];
        let d2 = dimensions[1];
        let d3 = dimensions[2];

        // Smallest side.
        total_paper += d1 * d2;

        // Surface area.
        total_paper += 2 * (d1*d2 + d1*d3 + d2*d3);

        // Wrapping ribbon.
        total_ribbon += 2 * (d1 + d2);

        // Bow ribbon.
        total_ribbon += d1 * d2 * d3;
    }

    println!("The total amount of paper needed is {} square feet.", total_paper);
    println!("The total amount of ribbon needed is {} square feet.", total_ribbon);
}
