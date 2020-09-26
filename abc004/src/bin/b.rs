use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let xs = input.lines().collect::<Vec<_>>();
    for i in (0..xs.len()).rev() {
        println!("{}", xs[i].chars().rev().collect::<String>());
    }
}
