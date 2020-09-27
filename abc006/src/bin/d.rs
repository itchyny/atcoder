use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let n = read!(usize);
    let mut xs: Vec<u64> = vec![];
    for _ in 0..n {
        let c = read!(u64);
        let i = xs.binary_search(&c).unwrap_or_else(|x| x);
        if i == xs.len() {
            xs.push(c)
        } else {
            xs[i] = c
        }
    }
    println!("{}", n - xs.len());
}
