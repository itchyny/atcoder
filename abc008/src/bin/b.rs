use std::collections::HashMap;
use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let mut xs = HashMap::new();
    for _ in 0..read!(usize) {
        *xs.entry(read!(String)).or_insert(0) += 1;
    }
    println!("{}", xs.iter().max_by_key(|&(x, cnt)| (cnt, x)).unwrap().0);
}
