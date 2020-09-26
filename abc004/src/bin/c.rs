use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let n = read!(usize);
    let mut xs = (0..6).collect::<Vec<_>>();
    let mut i = 0;
    while i < n {
        xs.swap(i % 5, i % 5 + 1);
        i += 1;
        if xs.iter().enumerate().all(|(i, &x)| i == x) {
            i = n - n % i;
        }
    }
    let ans = xs
        .iter()
        .map(|x| (x + 1).to_string())
        .collect::<Vec<_>>()
        .join("");

    println!("{}", ans);
}
