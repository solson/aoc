extern crate aoc;

const KEYPAD: [[Option<char>; 5]; 5] = [
    [None,      None,      Some('1'), None,      None],
    [None,      Some('2'), Some('3'), Some('4'), None],
    [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
    [None,      Some('A'), Some('B'), Some('C'), None],
    [None,      None,      Some('D'), None,      None],
];

/// Position labelling:
///
///  |0 1 2 3 4
/// -+---------
/// 0|    1
/// 1|  2 3 4
/// 2|5 6 7 8 9
/// 3|  A B C
/// 4|    D
///
/// Number "5" has position (0, 2).
fn position_to_number(position: (i64, i64)) -> Option<char> {
    KEYPAD
        .get(position.1 as usize)
        .and_then(|row| row.get(position.0 as usize))
        .and_then(|&cell| cell)
}

fn main() {
    let input = aoc::read_file("input/day02");
    let mut pos: (i64, i64) = (0, 2); // Start on "5".

    for line in input.trim().split("\n") {
        for direction in line.chars() {
            let new_pos = match direction {
                'U' => (pos.0, pos.1 - 1),
                'D' => (pos.0, pos.1 + 1),
                'L' => (pos.0 - 1, pos.1),
                'R' => (pos.0 + 1, pos.1),
                _ => panic!(),
            };

            if position_to_number(new_pos).is_some() {
                pos = new_pos;
            }

            // println!("{} to {}", direction, position_to_number(pos).unwrap());
        }

        print!("{}", position_to_number(pos).unwrap());
    }

    println!("");
}
