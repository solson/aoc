extern crate aoc;

use std::cell::RefCell;
use std::collections::HashMap;

type Wire = String;

#[derive(Clone, Debug)]
enum Input {
    Const(u16),
    Direct(Wire),
    And(Wire, Wire),
    Or(Wire, Wire),
    LShift(Wire, u16),
    RShift(Wire, u16),
    Not(Wire),
}

struct Circuit {
    connections: HashMap<Wire, Input>,
    values: RefCell<HashMap<Wire, u16>>,
}

impl Circuit {
    fn new() -> Self {
        Circuit {
            connections: HashMap::new(),
            values: RefCell::new(HashMap::new()),
        }
    }

    fn eval(&self, wire: &str) -> u16 {
        use Input::*;

        if let Some(&v) = self.values.borrow().get(wire) {
            return v;
        }

        let v = match self.connections[wire].clone() {
            Const(n)            => n,
            Direct(ref w)       => self.eval(w),
            And(ref w1, ref w2) => self.eval(w1) & self.eval(w2),
            Or(ref w1, ref w2)  => self.eval(w1) | self.eval(w2),
            LShift(ref w, n)    => self.eval(w) << n,
            RShift(ref w, n)    => self.eval(w) >> n,
            Not(ref w)          => !self.eval(w),
        };

        self.values.borrow_mut().insert(String::from(wire), v);

        v
    }
}

fn main() {
    use Input::*;

    let input = aoc::read_file("input/day07");
    let mut circuit = Circuit::new();

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let target = String::from(*words.last().unwrap());

        let input = match words[1] {
            "->"     => match words[0].parse::<u16>() {
                Ok(n)  => Const(n),
                Err(_) => Direct(String::from(words[0])),
            },
            "AND"    => And(String::from(words[0]), String::from(words[2])),
            "OR"     => Or(String::from(words[0]), String::from(words[2])),
            "LSHIFT" => LShift(String::from(words[0]), words[2].parse().unwrap()),
            "RSHIFT" => RShift(String::from(words[0]), words[2].parse().unwrap()),
            _        => Not(String::from(words[1])),
        };

        // println!("{} from {:?}", target, input);
        circuit.connections.insert(target, input);
    }

    // Hack to support the silly "1 AND b -> c" lines.
    circuit.connections.insert(String::from("1"), Const(1));

    let a = circuit.eval("a");
    println!("The value of wire 'a' is {}.", a);
    circuit.values.borrow_mut().clear();
    circuit.connections.insert(String::from("b"), Const(a));
    println!("The value of wire 'a' after overriding b is {}.", circuit.eval("a"));
}
