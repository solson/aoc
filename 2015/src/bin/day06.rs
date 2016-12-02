extern crate aoc;

fn main() {
    let input = aoc::read_file("input/day06");
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let mut grid = [[false; 1000]; 1000];

    for line in input.lines() {
        let mut words = line.split_whitespace();

        let new_value = match words.next().unwrap() {
            "turn" => match words.next().unwrap() {
                "off" => Some(false),
                "on" => Some(true),
                w => panic!("unexpected word: {}", w),
            },
            "toggle" => None,
            w => panic!("unexpected word: {}", w),
        };

        let start = parse_coordinate(words.next().unwrap());
        assert_eq!(words.next().unwrap(), "through");
        let end = parse_coordinate(words.next().unwrap());

        for i in start.0..end.0 + 1 {
            for j in start.1..end.1 + 1 {
                grid[i][j] = new_value.unwrap_or(!grid[i][j]);
            }
        }
    }

    let num_lights_lit = grid.iter().flat_map(|row| row.iter()).filter(|&&b| b).count();
    println!("There are {} lights lit.", num_lights_lit);
}

fn part_2(input: &str) {
    let mut grid = Box::new([[0isize; 1000]; 1000]);

    for line in input.lines() {
        let mut words = line.split_whitespace();

        let change = match words.next().unwrap() {
            "turn" => match words.next().unwrap() {
                "off" => -1,
                "on" => 1,
                w => panic!("unexpected word: {}", w),
            },
            "toggle" => 2,
            w => panic!("unexpected word: {}", w),
        };

        let start = parse_coordinate(words.next().unwrap());
        assert_eq!(words.next().unwrap(), "through");
        let end = parse_coordinate(words.next().unwrap());

        for i in start.0..end.0 + 1 {
            for j in start.1..end.1 + 1 {
                if change < 0 && grid[i][j] == 0 {
                    continue;
                }

                grid[i][j] += change;
            }
        }
    }

    let brightness = grid.iter().flat_map(|row| row.iter()).fold(0, |a, &b| a + b);
    println!("The total brightness is {}.", brightness);
}

fn parse_coordinate(s: &str) -> (usize, usize) {
    let mut iter = s.split(',').map(|n| n.parse().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}
