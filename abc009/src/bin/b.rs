use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ([$tt:tt]) => (read!([$tt; read!(usize)]));
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let xs = read!([u64]);
    let m = xs.iter().max().unwrap();
    println!("{}", xs.iter().filter(|&x| x != m).max().unwrap());
}
