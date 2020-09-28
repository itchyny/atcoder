use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    println!("{}", if line == "a" { "-1" } else { "a" });
}
