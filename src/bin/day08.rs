extern crate aoc;

fn main() {
    let input = aoc::read_file("input/day08");

    let mut code_len = 0;
    let mut memory_len = 0;
    let mut encoded_len = 0;

    for line in input.lines() {
        let mut chars = line[1..line.len()-1].chars();
        code_len += line.len();
        encoded_len += format!("{:?}", line).len();

        while let Some(c) = chars.next() {
            memory_len += 1;

            if c == '\\' {
                let c2 = chars.next().unwrap();
                if c2 == 'x' {
                    chars.next();
                    chars.next();
                }
            }
        }
    }

    println!("The source is {} bytes longer than the memory.", code_len - memory_len);
    println!("The encoded string are {} bytes longer than the source.", encoded_len - code_len);
}
