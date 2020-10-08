use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    println!("{}", line.as_bytes()[0] - b'A' + 1);
}
