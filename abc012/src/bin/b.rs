use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let n = read!(u64);
    println!("{:02}:{:02}:{:02}", n / 3600, n / 60 % 60, n % 60);
}
