extern crate aoc;
extern crate md5;

use std::io::Write;

fn main() {
    const KEY: &'static [u8] = b"bgvyzdsv";

    let mut i: u64 = 1;
    let mut vec = Vec::from(KEY);

    let mut five_zeroes = None;
    let mut six_zeroes = None;

    while five_zeroes.is_none() || six_zeroes.is_none() {
        write!(vec, "{}", i).unwrap();
        let digest = md5::compute(&vec);

        if five_zeroes.is_none() && digest[0..2] == [0; 2] && digest[2] <= 0x0F {
            five_zeroes = Some(i);
        }

        if six_zeroes.is_none() && digest[0..3] == [0; 3] {
            six_zeroes = Some(i);
        }

        vec.truncate(KEY.len());
        i += 1;
    }

    println!("The five-zeroes answer is {}.", five_zeroes.unwrap());
    println!("The six-zeroes answer is {}.", six_zeroes.unwrap());
}
