use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    println!(
        "{}",
        line.chars()
            .enumerate()
            .map(|(i, c)| if i == 0 {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            })
            .collect::<String>()
    );
}
