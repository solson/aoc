extern crate aoc;

fn increment(s: &mut [u8]) {
    let mut i = s.len() - 1;

    while s[i] == b'z' {
        if i == 0 {
            panic!("can't increment without adding new letter");
        }

        s[i] = b'a';
        i -= 1;
    }

    s[i] += 1;
}

fn valid_password(password: &[u8]) -> bool {
    let mut has_increasing_run = false;

    for w in password.windows(3) {
        if w[2] as i8 - w[1] as i8 == 1 && w[1] as i8 - w[0] as i8 == 1 {
            has_increasing_run = true;
        }
    }

    if !has_increasing_run { return false; }

    let mut has_pair = false;
    let mut iter = password.iter().cloned().peekable();
    while let Some(c1) = iter.next() {
        if c1 == b'i' || c1 == b'o' || c1 == b'l' {
            return false;
        }

        if let Some(&c2) = iter.peek() {
            if c1 == c2 {
                iter.next();
                if has_pair {
                    // This is the second pair.
                    return true;
                } else {
                    // This is the first pair. Keep looking.
                    has_pair = true;
                }
            }
        }
    }

    false
}

fn main() {
    const INPUT: &'static [u8] = b"hxbxwxba";
    let mut password = Vec::from(INPUT);

    for i in 0..2 {
        while !valid_password(&password) {
            increment(&mut password);
        }

        println!("Password {}: {}", i, std::str::from_utf8(&password).unwrap());
        increment(&mut password);
    }
}
