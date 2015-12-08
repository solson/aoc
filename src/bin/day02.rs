extern crate aoc;

fn main() {
    let input = aoc::read_file("input/day02");

    let mut total_paper = 0;
    let mut total_ribbon = 0;

    for line in input.lines() {
        let mut dimensions: Vec<i32> = line.split('x').map(|s| s.parse().unwrap()).collect();

        // Smallest side.
        dimensions.sort();
        total_paper += dimensions[0] * dimensions[1];

        // Surface area.
        total_paper += 2 * (dimensions[0] * dimensions[1] +
                            dimensions[0] * dimensions[2] +
                            dimensions[1] * dimensions[2]);

        // Wrapping ribbon.
        total_ribbon += 2 * (dimensions[0] + dimensions[1]);

        // Bow ribbon.
        total_ribbon += dimensions[0] * dimensions[1] * dimensions[2];
    }

    println!("The total amount of paper needed is {} square feet.", total_paper);
    println!("The total amount of ribbon needed is {} square feet.", total_ribbon);
}
