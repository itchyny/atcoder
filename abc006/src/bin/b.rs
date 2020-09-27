use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let n = read!(usize);
    let (mut a, mut b, mut c) = (0, 0, 1);
    for _ in 1..n {
        let d = (a + b + c) % 10007;
        a = b;
        b = c;
        c = d;
    }
    println!("{}", a);
}
