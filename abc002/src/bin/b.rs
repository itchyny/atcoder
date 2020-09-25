use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    println!(
        "{}",
        line.chars()
            .filter(|&c| c != 'a' && c != 'i' && c != 'u' && c != 'e' && c != 'o')
            .collect::<String>()
    );
}
